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

//for non windows systems, the path for folders and managed members' names will be UTF8, for windows it'll be regular UTF16
#if defined(_WIN32)
    //I hate C++ naming conventions
    typedef std::wstring string_t;
    #define STR(s) L ## s
    #define DIR_SEPARATOR L'\\'
    typedef std::wofstream output_stream;
#else
    #include <dlfcn.h>
    #include <limits.h>

    typedef std::string string_t;
    #define STR(s) s
    #define DIR_SEPARATOR '/'
    #define MAX_PATH PATH_MAX
    typedef std::ofstream output_stream;
#endif


HMODULE hm;

string_t exePath;

const string_t dllName = STR("XInput1_4");

output_stream file;

//I used a bunch of MessageBoxes when I was making this to debug and make sure everything worked fine and stuff
constexpr bool debugWithMessageBoxes = false;

//most of the following is from https://learn.microsoft.com/en-us/dotnet/core/tutorials/netcore-hosting, and https://github.com/dotnet/samples/blob/main/core/hosting/src/NativeHost/nativehost.cpp

// Globals to hold hostfxr exports
hostfxr_initialize_for_runtime_config_fn init_for_config_fptr;
hostfxr_get_runtime_delegate_fn get_delegate_fptr;
hostfxr_close_fn close_fptr;

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

	init_for_config_fptr = (hostfxr_initialize_for_runtime_config_fn)get_export(lib, "hostfxr_initialize_for_runtime_config");
	get_delegate_fptr = (hostfxr_get_runtime_delegate_fn)get_export(lib, "hostfxr_get_runtime_delegate");
	close_fptr = (hostfxr_close_fn)get_export(lib, "hostfxr_close");

	return (init_for_config_fptr && get_delegate_fptr && close_fptr);
}

load_assembly_and_get_function_pointer_fn get_dotnet_load_assembly(const char_t* config_path)
{
    int rc = 0;

	void* load_assembly_and_get_function_pointer = nullptr;

    //we don't have to pass a context handle, if there's already a running host, it'll automagically use its context pointer on its own
    rc = get_delegate_fptr(nullptr, hdt_load_assembly_and_get_function_pointer, &load_assembly_and_get_function_pointer);

    //if the status code is >= 0, success, we in business, otherwise, we have to initialise the host ourselves
    if (rc >= 0)
    {
        file << "skipping initialising host, there's already one running" << std::endl;

        return (load_assembly_and_get_function_pointer_fn)load_assembly_and_get_function_pointer;
    }

    hostfxr_handle cxt = nullptr;

	rc = init_for_config_fptr(config_path, nullptr, &cxt);

	//According to https://github.com/dotnet/runtime/blob/main/docs/design/features/host-error-codes.md, all returned codes below 0 are errors, and ones above are successes
	//We actually should get code 1, which means that another host is already loaded, but is compatible with ours, thus we basically get the handle to the host that the game uses
	if (rc < 0 || cxt == nullptr)
	{
		file << "Init failed: " << std::hex << std::showbase << rc << std::endl;
		close_fptr(cxt);
		return nullptr;
	}

#if defined (_WIN32)
	if (debugWithMessageBoxes)
	{
		MessageBox(0, std::to_wstring(rc).c_str(), L"MADE IT!!!!", MB_ICONASTERISK);
	}
#endif

	file << "HostFXR Context Handle: " << std::hex << std::showbase << (size_t)cxt << std::endl;

	rc = get_delegate_fptr(cxt, hdt_load_assembly_and_get_function_pointer, &load_assembly_and_get_function_pointer);

	if (rc != 0 || load_assembly_and_get_function_pointer == nullptr)
	{
		file << "Get delegate failed: " << std::hex << std::showbase << rc << std::endl;
	}

	return (load_assembly_and_get_function_pointer_fn)load_assembly_and_get_function_pointer;

}

void loadEntryPointMethod(load_assembly_and_get_function_pointer_fn load_assembly_and_get_function_pointer)
{
	char_t buffer[MAX_PATH] = { 0 };

	GetModuleFileName(NULL, buffer, MAX_PATH);

	string_t entrypointDllPath = std::filesystem::path(buffer).parent_path() / STR("BepInSbox") / STR("core") / STR("BepInSbox.NET.CoreCLR.dll");

	file << "full path for entrypoint Dll: " << entrypointDllPath << std::endl;

	//Name of [namespace (if there is one)].[class], [name of dll]
	const char_t* dotnet_type = STR("StartupHook, BepInSbox.NET.CoreCLR");

	//Name of method
	const char_t* dotnet_type_method = STR("Initialize");

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

#if defined (_WIN32)
	if (debugWithMessageBoxes)
	{
		MessageBox(0, std::to_wstring(rc).c_str(), L"RC", MB_ICONASTERISK);
		MessageBox(0, std::to_wstring((size_t)bootstrap).c_str(), L"bootstrap", MB_ICONASTERISK);
	}
#endif

	if (rc == 0 && bootstrap != nullptr)
	{
		bootstrap(nullptr, 0);
	}

}

void initNetCore(const string_t config_path)
{
#if defined (_WIN32)
	if (debugWithMessageBoxes)
	{
		MessageBox(0, TEXT("FROM INIT NET CORE"), TEXT("HI"), MB_ICONEXCLAMATION);
	}
#endif

	if (!load_hostfxr())
	{
		file << "failed to load host fxr :(" << std::endl;

#if defined (_WIN32)
		if (debugWithMessageBoxes)
		{
			MessageBox(0, TEXT("Failed"), TEXT("FAILED TO LOAD HOST FXR"), MB_ICONERROR);
		}
#endif

		return;
	}

	load_assembly_and_get_function_pointer_fn load_assembly_and_get_function_pointer = nullptr;

	load_assembly_and_get_function_pointer = get_dotnet_load_assembly(config_path.c_str());

#if defined (_WIN32)
	if (debugWithMessageBoxes)
	{
		MessageBox(0, std::to_wstring(load_assembly_and_get_function_pointer == nullptr).c_str(), L"Hi", MB_ICONASTERISK);
	}
#endif

	loadEntryPointMethod(load_assembly_and_get_function_pointer);
}

#if defined (_WIN32)
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

		string_t dllPath{ pString };

		dllPath += DIR_SEPARATOR + dllName + STR(".dll");

		xinput.LoadOriginalLibrary(LoadLibrary(dllPath.c_str()));

		file.open("DoorstopLog.txt");

        char str[26];

        std::time_t result = std::chrono::system_clock::to_time_t(std::chrono::system_clock::now());

        ctime_s(str, sizeof(str), &result);

		file << str << std::endl;

		if (!file.is_open())
		{
			std::cout << "Couldn't create log file" << std::endl;
			return false;
		}

		exePath.reserve(MAX_PATH);

		size_t size = 0;

		char_t fuck[MAX_PATH];

		if (FAILED(size = (GetModuleFileName(NULL, (LPWSTR)fuck, MAX_PATH))))
		{
			return false;
		}

		exePath = string_t(fuck);

		exePath.resize(size);

		file << "module filename: " << exePath << std::endl;

		if (debugWithMessageBoxes)
		{
			MessageBox(0, exePath.c_str(), L"here's the path", MB_ICONASTERISK);
		}

		std::filesystem::path path(exePath);

		file << "game exe full path: " << path.parent_path().wstring() << std::endl;

        if (path.filename() == "sbox.exe")
        {
            file << "current exe is sbox.exe, skipping loading shit, sowwie mxster facepunch " << path.parent_path().wstring() << std::endl;

            file.close();

            //sbox's editor is in the same folder as sbox.exe, so if we want to mod the editor and also play the game, we should just pretend like the load went fine and let the game go on its merry way ^^
            return true;
        }

		std::filesystem::path runtimeConfigPath = path.replace_filename(L"sbox-standalone.runtimeconfig.json");

		file << "runtime config full path: " << runtimeConfigPath << std::endl;

		initNetCore(runtimeConfigPath);

		file.close();

		return true;
	}
	else if (ul_reason_for_call == DLL_PROCESS_DETACH)
	{
		if (debugWithMessageBoxes)
		{
			MessageBox(0, TEXT("byee byereeee"), TEXT("Goodbye!!!!!!"), MB_ICONINFORMATION);
		}

        file << "process detached, goodbye" << std::endl;

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
#endif
