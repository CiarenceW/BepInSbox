#pragma once
#include "pch.h"
//taken from https://github.com/ThirteenAG/Ultimate-ASI-Loader/blob/master/source/dllmain.h

struct shared
{
	FARPROC DllCanUnloadNow;
	FARPROC DllGetClassObject;
	FARPROC DllRegisterServer;
	FARPROC DllUnregisterServer;
	FARPROC DebugSetMute;

	void LoadOriginalLibrary(HMODULE dll)
	{
		DllCanUnloadNow = GetProcAddress(dll, "DllCanUnloadNow");
		DllGetClassObject = GetProcAddress(dll, "DllGetClassObject");
		DllRegisterServer = GetProcAddress(dll, "DllRegisterServer");
		DllUnregisterServer = GetProcAddress(dll, "DllUnregisterServer");
		DebugSetMute = GetProcAddress(dll, "DebugSetMute");
	}
} shared;

struct xinput_dll
{
	HMODULE dll;
	FARPROC DllMain;
	FARPROC XInputEnable;
	FARPROC XInputGetCapabilities;
	FARPROC XInputGetDSoundAudioDeviceGuids;
	FARPROC XInputGetState;
	FARPROC XInputSetState;
	FARPROC XInputGetBatteryInformation;
	FARPROC XInputGetKeystroke;
	FARPROC XInputGetAudioDeviceIds;
	FARPROC XInputGetStateEx;
	FARPROC XInputWaitForGuideButton;
	FARPROC XInputCancelGuideButtonWait;
	FARPROC XInputPowerOffController;
	FARPROC XInputGetBaseBusInformation;
	FARPROC XInputGetCapabilitiesEx;

	void LoadOriginalLibrary(HMODULE module)
	{
		dll = module;
		DllMain = GetProcAddress(dll, "DllMain");
		XInputEnable = GetProcAddress(dll, "XInputEnable");
		XInputGetCapabilities = GetProcAddress(dll, "XInputGetCapabilities");
		XInputGetDSoundAudioDeviceGuids = GetProcAddress(dll, "XInputGetDSoundAudioDeviceGuids");
		XInputGetState = GetProcAddress(dll, "XInputGetState");
		XInputSetState = GetProcAddress(dll, "XInputSetState");
		XInputGetBatteryInformation = GetProcAddress(dll, "XInputGetBatteryInformation");
		XInputGetKeystroke = GetProcAddress(dll, "XInputGetKeystroke");
		XInputGetAudioDeviceIds = GetProcAddress(dll, "XInputGetAudioDeviceIds");
		XInputGetStateEx = GetProcAddress(dll, (LPCSTR)100);
		XInputWaitForGuideButton = GetProcAddress(dll, (LPCSTR)101);
		XInputCancelGuideButtonWait = GetProcAddress(dll, (LPCSTR)102);
		XInputPowerOffController = GetProcAddress(dll, (LPCSTR)103);
		XInputGetBaseBusInformation = GetProcAddress(dll, (LPCSTR)104);
		XInputGetCapabilitiesEx = GetProcAddress(dll, (LPCSTR)108);
	}
} xinput;

#pragma runtime_checks( "", off )

#ifdef _DEBUG
#pragma message ("You are compiling the code in Debug - be warned that wrappers for export functions may not have correct code generated")
#endif

void _DllRegisterServer() { shared.DllRegisterServer(); }
void _DllUnregisterServer() { shared.DllUnregisterServer(); }
void _DllCanUnloadNow() { shared.DllCanUnloadNow(); }
void _DllGetClassObject() { shared.DllGetClassObject(); }
void _DebugSetMute() { shared.DebugSetMute(); }

void _DllMain() { xinput.DllMain(); }
void _XInputEnable() { xinput.XInputEnable(); }
void _XInputGetCapabilities() { xinput.XInputGetCapabilities(); }
void _XInputGetDSoundAudioDeviceGuids() { xinput.XInputGetDSoundAudioDeviceGuids(); }
void _XInputGetState() { xinput.XInputGetState(); }
void _XInputSetState() { xinput.XInputSetState(); }
void _XInputGetBatteryInformation() { xinput.XInputGetBatteryInformation(); }
void _XInputGetKeystroke() { xinput.XInputGetKeystroke(); }
void _XInputGetStateEx() { xinput.XInputGetStateEx(); }
void _XInputWaitForGuideButton() { xinput.XInputWaitForGuideButton(); }
void _XInputCancelGuideButtonWait() { xinput.XInputCancelGuideButtonWait(); }
void _XInputPowerOffController() { xinput.XInputPowerOffController(); }
void _XInputGetAudioDeviceIds() { xinput.XInputGetAudioDeviceIds(); }
void _XInputGetBaseBusInformation() { xinput.XInputGetBaseBusInformation(); }
void _XInputGetCapabilitiesEx() { xinput.XInputGetCapabilitiesEx(); }

#pragma runtime_checks( "", restore )
