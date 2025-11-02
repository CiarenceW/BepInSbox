// dllmain.cpp : Defines the entry point for the DLL application.
#include "pch.h"
#include <thread>
#include <assert.h>
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <chrono>
#include <vector>
#include <filesystem>
#include <string>
#include <fstream>
#include <iostream>
#include <ShlObj_core.h>
#include <hostfxr.h>
#include <nethost.h>
#include <coreclr_delegates.h>
#include "dllmain.h"
#include <time.h>

HMODULE hm;

std::wstring exePath;

const std::wstring dllName = L"XInput1_4";

std::wofstream file;

constexpr bool debugWithMessageBoxes = false;

// Globals to hold hostfxr exports
hostfxr_initialize_for_dotnet_command_line_fn init_for_cmd_line_fptr;
hostfxr_initialize_for_runtime_config_fn init_for_config_fptr;
hostfxr_get_runtime_delegate_fn get_delegate_fptr;
hostfxr_run_app_fn run_app_fptr;
hostfxr_close_fn close_fptr;

hostfxr_handle cxt = nullptr;

void* load_library(const char_t* path)
{
	HMODULE h = ::LoadLibraryW(path);
	assert(h != nullptr);
	return (void*)h;
}
void* get_export(void* h, const char* name)
{
	void* f = ::GetProcAddress((HMODULE)h, name);
	assert(f != nullptr);
	return f;
}

bool load_hostfxr()
{
	char_t buffer[MAX_PATH];
	size_t buffer_size = sizeof(buffer) / sizeof(char_t);

	int rc = get_hostfxr_path(buffer, &buffer_size, nullptr);

	if (rc != 0)
	{
		return false;
	}

	void* lib = load_library(buffer);

	init_for_cmd_line_fptr = (hostfxr_initialize_for_dotnet_command_line_fn)get_export(lib, "hostfxr_initialize_for_dotnet_command_line");
	init_for_config_fptr = (hostfxr_initialize_for_runtime_config_fn)get_export(lib, "hostfxr_initialize_for_runtime_config");
	get_delegate_fptr = (hostfxr_get_runtime_delegate_fn)get_export(lib, "hostfxr_get_runtime_delegate");
	run_app_fptr = (hostfxr_run_app_fn)get_export(lib, "hostfxr_run_app");
	close_fptr = (hostfxr_close_fn)get_export(lib, "hostfxr_close");

	return (init_for_config_fptr && get_delegate_fptr && close_fptr);
}

load_assembly_and_get_function_pointer_fn get_dotnet_load_assembly(const char_t* config_path)
{
	//Load .NET Core
	void* load_assembly_and_get_function_pointer = nullptr;

	int rc = init_for_config_fptr(config_path, nullptr, &cxt);

	//According to https://github.com/dotnet/runtime/blob/main/docs/design/features/host-error-codes.md, all returned codes below 0 are errors, and ones above are successes
	//We actually should get code 1, which means that another host is already loaded, but is compatible with ours, thus we basically get the handle to the host that the game uses
	if (rc < 0 || cxt == nullptr)
	{
		file << "Init failed: " << std::hex << std::showbase << rc << std::endl;
		close_fptr(cxt);
		return nullptr;
	}

	if (debugWithMessageBoxes)
	{
		MessageBox(0, std::to_wstring(rc).c_str(), L"MADE IT!!!!", MB_ICONASTERISK);
	}

	file << "HostFXR Context Handle: " << std::hex << std::showbase << (unsigned long long)cxt << std::endl;

	//No loads refused
	rc = get_delegate_fptr(cxt, hdt_load_assembly_and_get_function_pointer, &load_assembly_and_get_function_pointer);

	if (rc != 0 || load_assembly_and_get_function_pointer == nullptr)
	{
		file << "Get delegate failed: " << std::hex << std::showbase << rc << std::endl;
	}

	return (load_assembly_and_get_function_pointer_fn)load_assembly_and_get_function_pointer;

}

void loadEntryPointMethod(load_assembly_and_get_function_pointer_fn load_assembly_and_get_function_pointer)
{
	wchar_t buffer[MAX_PATH] = { 0 };

	GetModuleFileName(NULL, buffer, MAX_PATH);

	std::wstring entrypointDllPath = std::filesystem::path(buffer).parent_path() / L"BepInSbox" / L"core" / L"BepInSbox.NET.CoreCLR.dll";

	file << "full path for entrypoint Dll: " << entrypointDllPath << std::endl;

	//Name of [namespace (if there is one)].[class], [name of dll]
	const wchar_t* dotnet_type = L"StartupHook, BepInSbox.NET.CoreCLR";

	//Name of method
	const wchar_t* dotnet_type_method = L"Initialize";

	component_entry_point_fn bootstrap = nullptr;

	int rc = load_assembly_and_get_function_pointer(
		entrypointDllPath.c_str(),
		dotnet_type,
		dotnet_type_method,
		UNMANAGEDCALLERSONLY_METHOD,
		nullptr,
		(void**)&bootstrap
	);

	file << "result code from load_assembly_and_get_function_pointer: " << std::hex << std::showbase << rc << std::endl;

	if (debugWithMessageBoxes)
	{
		MessageBox(0, std::to_wstring(rc).c_str(), L"RC", MB_ICONASTERISK);
		MessageBox(0, std::to_wstring((unsigned long long)bootstrap).c_str(), L"bootstrap", MB_ICONASTERISK);
	}

	if (rc == 0 && bootstrap != nullptr)
	{
		bootstrap(nullptr, 0);
	}

}

void initNetCore(const std::wstring config_path)
{
	if (debugWithMessageBoxes)
	{
		MessageBox(0, TEXT("FROM INIT NET CORE"), TEXT("HI"), MB_ICONEXCLAMATION);
	}

	if (!load_hostfxr())
	{
		file << "failed to load host fxr :(" << std::endl;

		if (debugWithMessageBoxes)
		{
			MessageBox(0, TEXT("Failed"), TEXT("FAILED TO LOAD HOST FXR FUCKKKKKKKKKKKKKK"), MB_ICONERROR);
		}

		return;
	}

	load_assembly_and_get_function_pointer_fn load_assembly_and_get_function_pointer = nullptr;

	load_assembly_and_get_function_pointer = get_dotnet_load_assembly(config_path.c_str());

	if (debugWithMessageBoxes)
	{
		MessageBox(0, std::to_wstring(load_assembly_and_get_function_pointer == nullptr).c_str(), L"Hi", MB_ICONASTERISK);
	}

	loadEntryPointMethod(load_assembly_and_get_function_pointer);
}

BOOL APIENTRY DllMain(HMODULE hModule,
	DWORD  ul_reason_for_call,
	LPVOID lpReserved
)
{
	if (debugWithMessageBoxes)
	{
		MessageBox(0, TEXT("hello"), std::to_wstring(ul_reason_for_call).c_str(), MB_ICONINFORMATION);
	}

	if (ul_reason_for_call == DLL_PROCESS_ATTACH)
	{
		PWSTR pString;

		if (FAILED(SHGetKnownFolderPath(FOLDERID_System, 0, NULL, &pString)))
		{
			return false;
		}

		std::wstring dllPath{ pString };

		dllPath += L'\\' + dllName + L".dll";

		xinput.LoadOriginalLibrary(LoadLibrary(dllPath.c_str()));

		file.open("DoorstopLog.txt");

        char str[26];

        std::time_t result = std::chrono::system_clock::to_time_t(std::chrono::system_clock::now());

        ctime_s(str, sizeof(str), &result);

		file << str << std::endl;

		if (!file.is_open())
		{
			std::cout << "FUCK FILE DIDN'T CREATE" << std::endl;
			return false;
		}

		exePath.reserve(MAX_PATH);

		size_t size = 0;

		wchar_t fuck[MAX_PATH];

		if (FAILED(size = (GetModuleFileName(NULL, (LPWSTR)fuck, MAX_PATH))))
		{
			return false;
		}

		file << "module filename: " << size << std::endl;

		exePath = std::wstring(fuck);

		exePath.resize(size);

		if (debugWithMessageBoxes)
		{
			MessageBox(0, exePath.c_str(), L"here's the path", MB_ICONASTERISK);
		}

		std::filesystem::path path(exePath);

		file << "game exe full path: " << path.parent_path().wstring() << std::endl;

        if (path.filename() == "sbox.exe")
        {
            MessageBox(0, L"Trying to use this with the real s&box? booooooooooooooooooooooooooooooooo", L"I'm cross at you", MB_ICONERROR | MB_OKCANCEL);
        }

		std::filesystem::path runtimeConfigPath = path.replace_filename(L"sbox-standalone.runtimeconfig.json");

		file << "runtime config full path: " << runtimeConfigPath.wstring() << std::endl;

		initNetCore(runtimeConfigPath.wstring());

		file.close();

		return true;
	}
	else if (ul_reason_for_call == DLL_PROCESS_DETACH)
	{
		if (debugWithMessageBoxes)
		{
			MessageBox(0, TEXT("byee byereeee"), TEXT("Goodbye!!!!!!"), MB_ICONINFORMATION);
		}

		return true;
	}
	else if (ul_reason_for_call == DLL_THREAD_ATTACH)
	{
		if (debugWithMessageBoxes)
		{
			MessageBox(0, TEXT("thread attached"), TEXT("hi new thread here"), MB_ICONINFORMATION);
		}

		return true;
	}
	else if (ul_reason_for_call == DLL_THREAD_DETACH)
	{
		if (debugWithMessageBoxes)
		{
			MessageBox(0, TEXT("thread go boomie"), TEXT("thread gone bye"), MB_ICONINFORMATION);
		}

		return true;
	}

	return true;
}
