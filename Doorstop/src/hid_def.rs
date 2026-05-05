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
static mut HidD_FlushQueue: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut HidD_FreePreparsedData: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut HidD_GetAttributes: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut HidD_GetConfiguration: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut HidD_GetFeature: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut HidD_GetHidGuid: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut HidD_GetIndexedString: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut HidD_GetInputReport: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut HidD_GetManufacturerString: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut HidD_GetMsGenreDescriptor: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut HidD_GetNumInputBuffers: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut HidD_GetPhysicalDescriptor: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut HidD_GetPreparsedData: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut HidD_GetProductString: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut HidD_GetSerialNumberString: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut HidD_Hello: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut HidD_SetConfiguration: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut HidD_SetFeature: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut HidD_SetNumInputBuffers: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut HidD_SetOutputReport: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut HidP_GetButtonArray: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut HidP_GetButtonCaps: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut HidP_GetCaps: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut HidP_GetData: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut HidP_GetExtendedAttributes: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut HidP_GetLinkCollectionNodes: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut HidP_GetScaledUsageValue: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut HidP_GetSpecificButtonCaps: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut HidP_GetSpecificValueCaps: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut HidP_GetUsageValue: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut HidP_GetUsageValueArray: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut HidP_GetUsages: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut HidP_GetUsagesEx: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut HidP_GetValueCaps: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut HidP_GetVersionInternal: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut HidP_InitializeReportForID: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut HidP_MaxDataListLength: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut HidP_MaxUsageListLength: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut HidP_SetButtonArray: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut HidP_SetData: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut HidP_SetScaledUsageValue: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut HidP_SetUsageValue: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut HidP_SetUsageValueArray: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut HidP_SetUsages: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut HidP_TranslateUsagesToI8042ScanCodes: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut HidP_UnsetUsages: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut HidP_UsageListDifference: FARPROC = None;


#[cfg(target_family = "windows")]
#[allow(non_snake_case)]
pub fn LoadOriginalLibrary(module: HMODULE)
{
    unsafe
    {
        dll = module;
        HidD_FlushQueue = GetProcAddress(dll, s!("HidD_FlushQueue"));
        HidD_FreePreparsedData = GetProcAddress(dll, s!("HidD_FreePreparsedData"));
        HidD_GetAttributes = GetProcAddress(dll, s!("HidD_GetAttributes"));
        HidD_GetConfiguration = GetProcAddress(dll, s!("HidD_GetConfiguration"));
        HidD_GetFeature = GetProcAddress(dll, s!("HidD_GetFeature"));
        HidD_GetHidGuid = GetProcAddress(dll, s!("HidD_GetHidGuid"));
        HidD_GetIndexedString = GetProcAddress(dll, s!("HidD_GetIndexedString"));
        HidD_GetInputReport = GetProcAddress(dll, s!("HidD_GetInputReport"));
        HidD_GetManufacturerString = GetProcAddress(dll, s!("HidD_GetManufacturerString"));
        HidD_GetMsGenreDescriptor = GetProcAddress(dll, s!("HidD_GetMsGenreDescriptor"));
        HidD_GetNumInputBuffers = GetProcAddress(dll, s!("HidD_GetNumInputBuffers"));
        HidD_GetPhysicalDescriptor = GetProcAddress(dll, s!("HidD_GetPhysicalDescriptor"));
        HidD_GetPreparsedData = GetProcAddress(dll, s!("HidD_GetPreparsedData"));
        HidD_GetProductString = GetProcAddress(dll, s!("HidD_GetProductString"));
        HidD_GetSerialNumberString = GetProcAddress(dll, s!("HidD_GetSerialNumberString"));
        HidD_Hello = GetProcAddress(dll, s!("HidD_Hello"));
        HidD_SetConfiguration = GetProcAddress(dll, s!("HidD_SetConfiguration"));
        HidD_SetFeature = GetProcAddress(dll, s!("HidD_SetFeature"));
        HidD_SetNumInputBuffers = GetProcAddress(dll, s!("HidD_SetNumInputBuffers"));
        HidD_SetOutputReport = GetProcAddress(dll, s!("HidD_SetOutputReport"));
        HidP_GetButtonArray = GetProcAddress(dll, s!("HidP_GetButtonArray"));
        HidP_GetButtonCaps = GetProcAddress(dll, s!("HidP_GetButtonCaps"));
        HidP_GetCaps = GetProcAddress(dll, s!("HidP_GetCaps"));
        HidP_GetData = GetProcAddress(dll, s!("HidP_GetData"));
        HidP_GetExtendedAttributes = GetProcAddress(dll, s!("HidP_GetExtendedAttributes"));
        HidP_GetLinkCollectionNodes = GetProcAddress(dll, s!("HidP_GetLinkCollectionNodes"));
        HidP_GetScaledUsageValue = GetProcAddress(dll, s!("HidP_GetScaledUsageValue"));
        HidP_GetSpecificButtonCaps = GetProcAddress(dll, s!("HidP_GetSpecificButtonCaps"));
        HidP_GetSpecificValueCaps = GetProcAddress(dll, s!("HidP_GetSpecificValueCaps"));
        HidP_GetUsageValue = GetProcAddress(dll, s!("HidP_GetUsageValue"));
        HidP_GetUsageValueArray = GetProcAddress(dll, s!("HidP_GetUsageValueArray"));
        HidP_GetUsages = GetProcAddress(dll, s!("HidP_GetUsages"));
        HidP_GetUsagesEx = GetProcAddress(dll, s!("HidP_GetUsagesEx"));
        HidP_GetValueCaps = GetProcAddress(dll, s!("HidP_GetValueCaps"));
        HidP_GetVersionInternal = GetProcAddress(dll, s!("HidP_GetVersionInternal"));
        HidP_InitializeReportForID = GetProcAddress(dll, s!("HidP_InitializeReportForID"));
        HidP_MaxDataListLength = GetProcAddress(dll, s!("HidP_MaxDataListLength"));
        HidP_MaxUsageListLength = GetProcAddress(dll, s!("HidP_MaxUsageListLength"));
        HidP_SetButtonArray = GetProcAddress(dll, s!("HidP_SetButtonArray"));
        HidP_SetData = GetProcAddress(dll, s!("HidP_SetData"));
        HidP_SetScaledUsageValue = GetProcAddress(dll, s!("HidP_SetScaledUsageValue"));
        HidP_SetUsageValue = GetProcAddress(dll, s!("HidP_SetUsageValue"));
        HidP_SetUsageValueArray = GetProcAddress(dll, s!("HidP_SetUsageValueArray"));
        HidP_SetUsages = GetProcAddress(dll, s!("HidP_SetUsages"));
        HidP_TranslateUsagesToI8042ScanCodes = GetProcAddress(dll, s!("HidP_TranslateUsagesToI8042ScanCodes"));
        HidP_UnsetUsages = GetProcAddress(dll, s!("HidP_UnsetUsages"));
        HidP_UsageListDifference = GetProcAddress(dll, s!("HidP_UsageListDifference"));
    }
}

#[unsafe(no_mangle)]
#[unsafe(export_name = "HidD_FlushQueue")]
pub extern "C" fn _HidD_FlushQueue() { unsafe { HidD_FlushQueue.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "HidD_FreePreparsedData")]
pub extern "C" fn _HidD_FreePreparsedData() { unsafe { HidD_FreePreparsedData.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "HidD_GetAttributes")]
pub extern "C" fn _HidD_GetAttributes() { unsafe { HidD_GetAttributes.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "HidD_GetConfiguration")]
pub extern "C" fn _HidD_GetConfiguration() { unsafe { HidD_GetConfiguration.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "HidD_GetFeature")]
pub extern "C" fn _HidD_GetFeature() { unsafe { HidD_GetFeature.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "HidD_GetHidGuid")]
pub extern "C" fn _HidD_GetHidGuid() { unsafe { HidD_GetHidGuid.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "HidD_GetIndexedString")]
pub extern "C" fn _HidD_GetIndexedString() { unsafe { HidD_GetIndexedString.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "HidD_GetInputReport")]
pub extern "C" fn _HidD_GetInputReport() { unsafe { HidD_GetInputReport.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "HidD_GetManufacturerString")]
pub extern "C" fn _HidD_GetManufacturerString() { unsafe { HidD_GetManufacturerString.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "HidD_GetMsGenreDescriptor")]
pub extern "C" fn _HidD_GetMsGenreDescriptor() { unsafe { HidD_GetMsGenreDescriptor.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "HidD_GetNumInputBuffers")]
pub extern "C" fn _HidD_GetNumInputBuffers() { unsafe { HidD_GetNumInputBuffers.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "HidD_GetPhysicalDescriptor")]
pub extern "C" fn _HidD_GetPhysicalDescriptor() { unsafe { HidD_GetPhysicalDescriptor.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "HidD_GetPreparsedData")]
pub extern "C" fn _HidD_GetPreparsedData() { unsafe { HidD_GetPreparsedData.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "HidD_GetProductString")]
pub extern "C" fn _HidD_GetProductString() { unsafe { HidD_GetProductString.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "HidD_GetSerialNumberString")]
pub extern "C" fn _HidD_GetSerialNumberString() { unsafe { HidD_GetSerialNumberString.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "HidD_Hello")]
pub extern "C" fn _HidD_Hello() { unsafe { HidD_Hello.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "HidD_SetConfiguration")]
pub extern "C" fn _HidD_SetConfiguration() { unsafe { HidD_SetConfiguration.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "HidD_SetFeature")]
pub extern "C" fn _HidD_SetFeature() { unsafe { HidD_SetFeature.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "HidD_SetNumInputBuffers")]
pub extern "C" fn _HidD_SetNumInputBuffers() { unsafe { HidD_SetNumInputBuffers.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "HidD_SetOutputReport")]
pub extern "C" fn _HidD_SetOutputReport() { unsafe { HidD_SetOutputReport.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "HidP_GetButtonArray")]
pub extern "C" fn _HidP_GetButtonArray() { unsafe { HidP_GetButtonArray.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "HidP_GetButtonCaps")]
pub extern "C" fn _HidP_GetButtonCaps() { unsafe { HidP_GetButtonCaps.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "HidP_GetCaps")]
pub extern "C" fn _HidP_GetCaps() { unsafe { HidP_GetCaps.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "HidP_GetData")]
pub extern "C" fn _HidP_GetData() { unsafe { HidP_GetData.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "HidP_GetExtendedAttributes")]
pub extern "C" fn _HidP_GetExtendedAttributes() { unsafe { HidP_GetExtendedAttributes.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "HidP_GetLinkCollectionNodes")]
pub extern "C" fn _HidP_GetLinkCollectionNodes() { unsafe { HidP_GetLinkCollectionNodes.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "HidP_GetScaledUsageValue")]
pub extern "C" fn _HidP_GetScaledUsageValue() { unsafe { HidP_GetScaledUsageValue.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "HidP_GetSpecificButtonCaps")]
pub extern "C" fn _HidP_GetSpecificButtonCaps() { unsafe { HidP_GetSpecificButtonCaps.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "HidP_GetSpecificValueCaps")]
pub extern "C" fn _HidP_GetSpecificValueCaps() { unsafe { HidP_GetSpecificValueCaps.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "HidP_GetUsageValue")]
pub extern "C" fn _HidP_GetUsageValue() { unsafe { HidP_GetUsageValue.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "HidP_GetUsageValueArray")]
pub extern "C" fn _HidP_GetUsageValueArray() { unsafe { HidP_GetUsageValueArray.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "HidP_GetUsages")]
pub extern "C" fn _HidP_GetUsages() { unsafe { HidP_GetUsages.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "HidP_GetUsagesEx")]
pub extern "C" fn _HidP_GetUsagesEx() { unsafe { HidP_GetUsagesEx.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "HidP_GetValueCaps")]
pub extern "C" fn _HidP_GetValueCaps() { unsafe { HidP_GetValueCaps.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "HidP_GetVersionInternal")]
pub extern "C" fn _HidP_GetVersionInternal() { unsafe { HidP_GetVersionInternal.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "HidP_InitializeReportForID")]
pub extern "C" fn _HidP_InitializeReportForID() { unsafe { HidP_InitializeReportForID.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "HidP_MaxDataListLength")]
pub extern "C" fn _HidP_MaxDataListLength() { unsafe { HidP_MaxDataListLength.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "HidP_MaxUsageListLength")]
pub extern "C" fn _HidP_MaxUsageListLength() { unsafe { HidP_MaxUsageListLength.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "HidP_SetButtonArray")]
pub extern "C" fn _HidP_SetButtonArray() { unsafe { HidP_SetButtonArray.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "HidP_SetData")]
pub extern "C" fn _HidP_SetData() { unsafe { HidP_SetData.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "HidP_SetScaledUsageValue")]
pub extern "C" fn _HidP_SetScaledUsageValue() { unsafe { HidP_SetScaledUsageValue.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "HidP_SetUsageValue")]
pub extern "C" fn _HidP_SetUsageValue() { unsafe { HidP_SetUsageValue.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "HidP_SetUsageValueArray")]
pub extern "C" fn _HidP_SetUsageValueArray() { unsafe { HidP_SetUsageValueArray.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "HidP_SetUsages")]
pub extern "C" fn _HidP_SetUsages() { unsafe { HidP_SetUsages.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "HidP_TranslateUsagesToI8042ScanCodes")]
pub extern "C" fn _HidP_TranslateUsagesToI8042ScanCodes() { unsafe { HidP_TranslateUsagesToI8042ScanCodes.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "HidP_UnsetUsages")]
pub extern "C" fn _HidP_UnsetUsages() { unsafe { HidP_UnsetUsages.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "HidP_UsageListDifference")]
pub extern "C" fn _HidP_UsageListDifference() { unsafe { HidP_UsageListDifference.unwrap()(); } }
