#[cfg(target_family = "windows")]
use windows::{
	core::*, 
    Win32::{
        System::Com::*,
        UI::WindowsAndMessaging::*,
		UI::Shell::*,
		Foundation::*,
		System::LibraryLoader::*,
		System::SystemServices::*,
    }
};

#[allow(non_upper_case_globals)]
static mut dll: HMODULE = unsafe { std::mem::transmute(0i64) };

#[allow(non_upper_case_globals)]
static mut DllMain: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut XInputEnable: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut XInputGetCapabilities: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut XInputGetDSoundAudioDeviceGuids: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut XInputGetState: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut XInputSetState: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut XInputGetBatteryInformation: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut XInputGetKeystroke: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut XInputGetAudioDeviceIds: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut XInputGetStateEx: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut XInputWaitForGuideButton: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut XInputCancelGuideButtonWait: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut XInputPowerOffController: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut XInputGetBaseBusInformation: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut XInputGetCapabilitiesEx: FARPROC = None;

#[cfg(target_family = "windows")]
#[allow(non_snake_case)]
pub fn LoadOriginalLibrary(module: HMODULE)
{
    unsafe
    {
        dll = module;
        DllMain = GetProcAddress(dll, s!("DllMain"));
		XInputEnable = GetProcAddress(dll, s!("XInputEnable"));
		XInputGetCapabilities = GetProcAddress(dll, s!("XInputGetCapabilities"));
		XInputGetDSoundAudioDeviceGuids = GetProcAddress(dll, s!("XInputGetDSoundAudioDeviceGuids"));
		XInputGetState = GetProcAddress(dll, s!("XInputGetState"));
		XInputSetState = GetProcAddress(dll, s!("XInputSetState"));
		XInputGetBatteryInformation = GetProcAddress(dll, s!("XInputGetBatteryInformation"));
		XInputGetKeystroke = GetProcAddress(dll, s!("XInputGetKeystroke"));
		XInputGetAudioDeviceIds = GetProcAddress(dll, s!("XInputGetAudioDeviceIds"));
		XInputGetStateEx = GetProcAddress(dll, std::mem::transmute::<i64, PCSTR>(100i64));
		XInputWaitForGuideButton = GetProcAddress(dll, std::mem::transmute::<i64, PCSTR>(101i64));
		XInputCancelGuideButtonWait = GetProcAddress(dll, std::mem::transmute::<i64, PCSTR>(102i64));
		XInputPowerOffController = GetProcAddress(dll, std::mem::transmute::<i64, PCSTR>(103i64));
		XInputGetBaseBusInformation = GetProcAddress(dll, std::mem::transmute::<i64, PCSTR>(104i64));
		XInputGetCapabilitiesEx = GetProcAddress(dll, std::mem::transmute::<i64, PCSTR>(108i64));
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn _DllMain() { unsafe { DllMain.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "XInputEnable")]
pub unsafe extern "C" fn _XInputEnable() { unsafe { XInputEnable.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "XInputGetCapabilities")]
pub unsafe extern "C" fn _XInputGetCapabilities() { unsafe { XInputGetCapabilities.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "XInputGetDSoundAudioDeviceGuids")]
pub unsafe extern "C" fn _XInputGetDSoundAudioDeviceGuids() { unsafe { XInputGetDSoundAudioDeviceGuids.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "XInputGetState")]
pub unsafe extern "C" fn _XInputGetState() { unsafe { XInputGetState.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "XInputSetState")]
pub unsafe extern "C" fn _XInputSetState() { unsafe { XInputSetState.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "XInputGetBatteryInformation")]
pub unsafe extern "C" fn _XInputGetBatteryInformation() { unsafe { XInputGetBatteryInformation.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "XInputGetKeystroke")]
pub unsafe extern "C" fn _XInputGetKeystroke() { unsafe { XInputGetKeystroke.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "XInputGetStateEx")]
pub unsafe extern "C" fn _XInputGetStateEx() { unsafe { XInputGetStateEx.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "XInputWaitForGuideButton")]
pub unsafe extern "C" fn _XInputWaitForGuideButton() { unsafe { XInputWaitForGuideButton.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "XInputCancelGuideButtonWait")]
pub unsafe extern "C" fn _XInputCancelGuideButtonWait() { unsafe { XInputCancelGuideButtonWait.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "XInputPowerOffController")]
pub unsafe extern "C" fn _XInputPowerOffController() { unsafe { XInputPowerOffController.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "XInputGetAudioDeviceIds")]
pub unsafe extern "C" fn _XInputGetAudioDeviceIds() { unsafe { XInputGetAudioDeviceIds.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "XInputGetBaseBusInformation")]
pub unsafe extern "C" fn _XInputGetBaseBusInformation() { unsafe { XInputGetBaseBusInformation.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "XInputGetCapabilitiesEx")]
pub unsafe extern "C" fn _XInputGetCapabilitiesEx() { unsafe { XInputGetCapabilitiesEx.unwrap()(); } }

