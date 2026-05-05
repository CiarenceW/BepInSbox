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
static mut DllCanUnloadNow: FARPROC = None;
 
#[allow(non_upper_case_globals)]
static mut DllGetClassObject: FARPROC = None;
 
#[allow(non_upper_case_globals)]
static mut Private1: FARPROC = None;
 
#[allow(non_upper_case_globals)]
static mut SvchostPushServiceGlobals: FARPROC = None;
 
#[allow(non_upper_case_globals)]
static mut WinHttpAddRequestHeaders: FARPROC = None;
 
#[allow(non_upper_case_globals)]
static mut WinHttpAddRequestHeadersEx: FARPROC = None;
 
#[allow(non_upper_case_globals)]
static mut WinHttpAutoProxySvcMain: FARPROC = None;
 
#[allow(non_upper_case_globals)]
static mut WinHttpCheckPlatform: FARPROC = None;
 
#[allow(non_upper_case_globals)]
static mut WinHttpCloseHandle: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpConnect: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpConnectionDeletePolicyEntries: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpConnectionDeleteProxyInfo: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpConnectionFreeNameList: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpConnectionFreeProxyInfo: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpConnectionFreeProxyList: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpConnectionGetNameList: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpConnectionGetProxyInfo: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpConnectionGetProxyList: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpConnectionOnlyConvert: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpConnectionOnlyReceive: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpConnectionOnlySend: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpConnectionSetPolicyEntries: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpConnectionSetProxyInfo: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpConnectionUpdateIfIndexTable: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpCrackUrl: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpCreateProxyList: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpCreateProxyManager: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpCreateProxyResolver: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpCreateProxyResult: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpCreateUiCompatibleProxyString: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpCreateUrl: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpDetectAutoProxyConfigUrl: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpFreeProxyResult: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpFreeProxyResultEx: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpFreeProxySettings: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpFreeProxySettingsEx: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpFreeQueryConnectionGroupResult: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpGetDefaultProxyConfiguration: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpGetIEProxyConfigForCurrentUser: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpGetProxyForUrl: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpGetProxyForUrlEx: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpGetProxyForUrlEx2: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpGetProxyForUrlHvsi: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpGetProxyResult: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpGetProxyResultEx: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpGetProxySettingsEx: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpGetProxySettingsResultEx: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpGetProxySettingsVersion: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpGetTunnelSocket: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpOpen: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpOpenRequest: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpPacJsWorkerMain: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpProbeConnectivity: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpProtocolCompleteUpgrade: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpProtocolReceive: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpProtocolSend: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpQueryAuthSchemes: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpQueryConnectionGroup: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpQueryDataAvailable: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpQueryHeaders: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpQueryHeadersEx: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpQueryOption: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpReadData: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpReadDataEx: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpReadProxySettings: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpReadProxySettingsHvsi: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpReceiveResponse: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpRefreshProxySettings: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpRegisterProxyChangeNotification: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpResetAutoProxy: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpResolverGetProxyForUrl: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpSaveProxyCredentials: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpSendRequest: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpSetCredentials: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpSetDefaultProxyConfiguration: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpSetOption: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpSetProxySettingsPerUser: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpSetSecureLegacyServersAppCompat: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpSetStatusCallback: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpSetTimeouts: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpTimeFromSystemTime: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpTimeToSystemTime: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpUnregisterProxyChangeNotification: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpWebSocketClose: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpWebSocketCompleteUpgrade: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpWebSocketQueryCloseStatus: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpWebSocketReceive: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpWebSocketSend: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpWebSocketShutdown: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpWriteData: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut WinHttpWriteProxySettings: FARPROC = None;

#[cfg(target_family = "windows")]
#[allow(non_snake_case)]
pub fn LoadOriginalLibrary(module: HMODULE)
{
    unsafe
    {
        dll = module;
        DllCanUnloadNow = GetProcAddress(dll, s!("DllCanUnloadNow"));
        DllGetClassObject = GetProcAddress(dll, s!("DllGetClassObject"));
        Private1 = GetProcAddress(dll, s!("Private1"));
        SvchostPushServiceGlobals = GetProcAddress(dll, s!("SvchostPushServiceGlobals"));
        WinHttpAddRequestHeaders = GetProcAddress(dll, s!("WinHttpAddRequestHeaders"));
        WinHttpAddRequestHeadersEx = GetProcAddress(dll, s!("WinHttpAddRequestHeadersEx"));
        WinHttpAutoProxySvcMain = GetProcAddress(dll, s!("WinHttpAutoProxySvcMain"));
        WinHttpCheckPlatform = GetProcAddress(dll, s!("WinHttpCheckPlatform"));
        WinHttpCloseHandle = GetProcAddress(dll, s!("WinHttpCloseHandle"));
        WinHttpConnect = GetProcAddress(dll, s!("WinHttpConnect"));
        WinHttpConnectionDeletePolicyEntries = GetProcAddress(dll, s!("WinHttpConnectionDeletePolicyEntries"));
        WinHttpConnectionDeleteProxyInfo = GetProcAddress(dll, s!("WinHttpConnectionDeleteProxyInfo"));
        WinHttpConnectionFreeNameList = GetProcAddress(dll, s!("WinHttpConnectionFreeNameList"));
        WinHttpConnectionFreeProxyInfo = GetProcAddress(dll, s!("WinHttpConnectionFreeProxyInfo"));
        WinHttpConnectionFreeProxyList = GetProcAddress(dll, s!("WinHttpConnectionFreeProxyList"));
        WinHttpConnectionGetNameList = GetProcAddress(dll, s!("WinHttpConnectionGetNameList"));
        WinHttpConnectionGetProxyInfo = GetProcAddress(dll, s!("WinHttpConnectionGetProxyInfo"));
        WinHttpConnectionGetProxyList = GetProcAddress(dll, s!("WinHttpConnectionGetProxyList"));
        WinHttpConnectionOnlyConvert = GetProcAddress(dll, s!("WinHttpConnectionOnlyConvert"));
        WinHttpConnectionOnlyReceive = GetProcAddress(dll, s!("WinHttpConnectionOnlyReceive"));
        WinHttpConnectionOnlySend = GetProcAddress(dll, s!("WinHttpConnectionOnlySend"));
        WinHttpConnectionSetPolicyEntries = GetProcAddress(dll, s!("WinHttpConnectionSetPolicyEntries"));
        WinHttpConnectionSetProxyInfo = GetProcAddress(dll, s!("WinHttpConnectionSetProxyInfo"));
        WinHttpConnectionUpdateIfIndexTable = GetProcAddress(dll, s!("WinHttpConnectionUpdateIfIndexTable"));
        WinHttpCrackUrl = GetProcAddress(dll, s!("WinHttpCrackUrl"));
        WinHttpCreateProxyList = GetProcAddress(dll, s!("WinHttpCreateProxyList"));
        WinHttpCreateProxyManager = GetProcAddress(dll, s!("WinHttpCreateProxyManager"));
        WinHttpCreateProxyResolver = GetProcAddress(dll, s!("WinHttpCreateProxyResolver"));
        WinHttpCreateProxyResult = GetProcAddress(dll, s!("WinHttpCreateProxyResult"));
        WinHttpCreateUiCompatibleProxyString = GetProcAddress(dll, s!("WinHttpCreateUiCompatibleProxyString"));
        WinHttpCreateUrl = GetProcAddress(dll, s!("WinHttpCreateUrl"));
        WinHttpDetectAutoProxyConfigUrl = GetProcAddress(dll, s!("WinHttpDetectAutoProxyConfigUrl"));
        WinHttpFreeProxyResult = GetProcAddress(dll, s!("WinHttpFreeProxyResult"));
        WinHttpFreeProxyResultEx = GetProcAddress(dll, s!("WinHttpFreeProxyResultEx"));
        WinHttpFreeProxySettings = GetProcAddress(dll, s!("WinHttpFreeProxySettings"));
        WinHttpFreeProxySettingsEx = GetProcAddress(dll, s!("WinHttpFreeProxySettingsEx"));
        WinHttpFreeQueryConnectionGroupResult = GetProcAddress(dll, s!("WinHttpFreeQueryConnectionGroupResult"));
        WinHttpGetDefaultProxyConfiguration = GetProcAddress(dll, s!("WinHttpGetDefaultProxyConfiguration"));
        WinHttpGetIEProxyConfigForCurrentUser = GetProcAddress(dll, s!("WinHttpGetIEProxyConfigForCurrentUser"));
        WinHttpGetProxyForUrl = GetProcAddress(dll, s!("WinHttpGetProxyForUrl"));
        WinHttpGetProxyForUrlEx = GetProcAddress(dll, s!("WinHttpGetProxyForUrlEx"));
        WinHttpGetProxyForUrlEx2 = GetProcAddress(dll, s!("WinHttpGetProxyForUrlEx2"));
        WinHttpGetProxyForUrlHvsi = GetProcAddress(dll, s!("WinHttpGetProxyForUrlHvsi"));
        WinHttpGetProxyResult = GetProcAddress(dll, s!("WinHttpGetProxyResult"));
        WinHttpGetProxyResultEx = GetProcAddress(dll, s!("WinHttpGetProxyResultEx"));
        WinHttpGetProxySettingsEx = GetProcAddress(dll, s!("WinHttpGetProxySettingsEx"));
        WinHttpGetProxySettingsResultEx = GetProcAddress(dll, s!("WinHttpGetProxySettingsResultEx"));
        WinHttpGetProxySettingsVersion = GetProcAddress(dll, s!("WinHttpGetProxySettingsVersion"));
        WinHttpGetTunnelSocket = GetProcAddress(dll, s!("WinHttpGetTunnelSocket"));
        WinHttpOpen = GetProcAddress(dll, s!("WinHttpOpen"));
        WinHttpOpenRequest = GetProcAddress(dll, s!("WinHttpOpenRequest"));
        WinHttpPacJsWorkerMain = GetProcAddress(dll, s!("WinHttpPacJsWorkerMain"));
        WinHttpProbeConnectivity = GetProcAddress(dll, s!("WinHttpProbeConnectivity"));
        WinHttpProtocolCompleteUpgrade = GetProcAddress(dll, s!("WinHttpProtocolCompleteUpgrade"));
        WinHttpProtocolReceive = GetProcAddress(dll, s!("WinHttpProtocolReceive"));
        WinHttpProtocolSend = GetProcAddress(dll, s!("WinHttpProtocolSend"));
        WinHttpQueryAuthSchemes = GetProcAddress(dll, s!("WinHttpQueryAuthSchemes"));
        WinHttpQueryConnectionGroup = GetProcAddress(dll, s!("WinHttpQueryConnectionGroup"));
        WinHttpQueryDataAvailable = GetProcAddress(dll, s!("WinHttpQueryDataAvailable"));
        WinHttpQueryHeaders = GetProcAddress(dll, s!("WinHttpQueryHeaders"));
        WinHttpQueryHeadersEx = GetProcAddress(dll, s!("WinHttpQueryHeadersEx"));
        WinHttpQueryOption = GetProcAddress(dll, s!("WinHttpQueryOption"));
        WinHttpReadData = GetProcAddress(dll, s!("WinHttpReadData"));
        WinHttpReadDataEx = GetProcAddress(dll, s!("WinHttpReadDataEx"));
        WinHttpReadProxySettings = GetProcAddress(dll, s!("WinHttpReadProxySettings"));
        WinHttpReadProxySettingsHvsi = GetProcAddress(dll, s!("WinHttpReadProxySettingsHvsi"));
        WinHttpReceiveResponse = GetProcAddress(dll, s!("WinHttpReceiveResponse"));
        WinHttpRefreshProxySettings = GetProcAddress(dll, s!("WinHttpRefreshProxySettings"));
        WinHttpRegisterProxyChangeNotification = GetProcAddress(dll, s!("WinHttpRegisterProxyChangeNotification"));
        WinHttpResetAutoProxy = GetProcAddress(dll, s!("WinHttpResetAutoProxy"));
        WinHttpResolverGetProxyForUrl = GetProcAddress(dll, s!("WinHttpResolverGetProxyForUrl"));
        WinHttpSaveProxyCredentials = GetProcAddress(dll, s!("WinHttpSaveProxyCredentials"));
        WinHttpSendRequest = GetProcAddress(dll, s!("WinHttpSendRequest"));
        WinHttpSetCredentials = GetProcAddress(dll, s!("WinHttpSetCredentials"));
        WinHttpSetDefaultProxyConfiguration = GetProcAddress(dll, s!("WinHttpSetDefaultProxyConfiguration"));
        WinHttpSetOption = GetProcAddress(dll, s!("WinHttpSetOption"));
        WinHttpSetProxySettingsPerUser = GetProcAddress(dll, s!("WinHttpSetProxySettingsPerUser"));
        WinHttpSetSecureLegacyServersAppCompat = GetProcAddress(dll, s!("WinHttpSetSecureLegacyServersAppCompat"));
        WinHttpSetStatusCallback = GetProcAddress(dll, s!("WinHttpSetStatusCallback"));
        WinHttpSetTimeouts = GetProcAddress(dll, s!("WinHttpSetTimeouts"));
        WinHttpTimeFromSystemTime = GetProcAddress(dll, s!("WinHttpTimeFromSystemTime"));
        WinHttpTimeToSystemTime = GetProcAddress(dll, s!("WinHttpTimeToSystemTime"));
        WinHttpUnregisterProxyChangeNotification = GetProcAddress(dll, s!("WinHttpUnregisterProxyChangeNotification"));
        WinHttpWebSocketClose = GetProcAddress(dll, s!("WinHttpWebSocketClose"));
        WinHttpWebSocketCompleteUpgrade = GetProcAddress(dll, s!("WinHttpWebSocketCompleteUpgrade"));
        WinHttpWebSocketQueryCloseStatus = GetProcAddress(dll, s!("WinHttpWebSocketQueryCloseStatus"));
        WinHttpWebSocketReceive = GetProcAddress(dll, s!("WinHttpWebSocketReceive"));
        WinHttpWebSocketSend = GetProcAddress(dll, s!("WinHttpWebSocketSend"));
        WinHttpWebSocketShutdown = GetProcAddress(dll, s!("WinHttpWebSocketShutdown"));
        WinHttpWriteData = GetProcAddress(dll, s!("WinHttpWriteData"));
        WinHttpWriteProxySettings = GetProcAddress(dll, s!("WinHttpWriteProxySettings"));
    }
}

#[unsafe(no_mangle)]
#[unsafe(export_name = "DllCanUnloadNow")]
pub unsafe extern "C" fn _DllCanUnloadNow() { unsafe { DllCanUnloadNow.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "DllGetClassObject")]
pub unsafe extern "C" fn _DllGetClassObject() { unsafe { DllGetClassObject.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "Private1")]
pub unsafe extern "C" fn _Private1() { unsafe { Private1.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "SvchostPushServiceGlobals")]
pub unsafe extern "C" fn _SvchostPushServiceGlobals() { unsafe { SvchostPushServiceGlobals.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpAddRequestHeaders")]
pub unsafe extern "C" fn _WinHttpAddRequestHeaders() { unsafe { WinHttpAddRequestHeaders.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpAddRequestHeadersEx")]
pub unsafe extern "C" fn _WinHttpAddRequestHeadersEx() { unsafe { WinHttpAddRequestHeadersEx.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpAutoProxySvcMain")]
pub unsafe extern "C" fn _WinHttpAutoProxySvcMain() { unsafe { WinHttpAutoProxySvcMain.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpCheckPlatform")]
pub unsafe extern "C" fn _WinHttpCheckPlatform() { unsafe { WinHttpCheckPlatform.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpCloseHandle")]
pub unsafe extern "C" fn _WinHttpCloseHandle() { unsafe { WinHttpCloseHandle.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpConnect")]
pub unsafe extern "C" fn _WinHttpConnect() { unsafe { WinHttpConnect.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpConnectionDeletePolicyEntries")]
pub unsafe extern "C" fn _WinHttpConnectionDeletePolicyEntries() { unsafe { WinHttpConnectionDeletePolicyEntries.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpConnectionDeleteProxyInfo")]
pub unsafe extern "C" fn _WinHttpConnectionDeleteProxyInfo() { unsafe { WinHttpConnectionDeleteProxyInfo.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpConnectionFreeNameList")]
pub unsafe extern "C" fn _WinHttpConnectionFreeNameList() { unsafe { WinHttpConnectionFreeNameList.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpConnectionFreeProxyInfo")]
pub unsafe extern "C" fn _WinHttpConnectionFreeProxyInfo() { unsafe { WinHttpConnectionFreeProxyInfo.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpConnectionFreeProxyList")]
pub unsafe extern "C" fn _WinHttpConnectionFreeProxyList() { unsafe { WinHttpConnectionFreeProxyList.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpConnectionGetNameList")]
pub unsafe extern "C" fn _WinHttpConnectionGetNameList() { unsafe { WinHttpConnectionGetNameList.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpConnectionGetProxyInfo")]
pub unsafe extern "C" fn _WinHttpConnectionGetProxyInfo() { unsafe { WinHttpConnectionGetProxyInfo.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpConnectionGetProxyList")]
pub unsafe extern "C" fn _WinHttpConnectionGetProxyList() { unsafe { WinHttpConnectionGetProxyList.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpConnectionOnlyConvert")]
pub unsafe extern "C" fn _WinHttpConnectionOnlyConvert() { unsafe { WinHttpConnectionOnlyConvert.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpConnectionOnlyReceive")]
pub unsafe extern "C" fn _WinHttpConnectionOnlyReceive() { unsafe { WinHttpConnectionOnlyReceive.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpConnectionOnlySend")]
pub unsafe extern "C" fn _WinHttpConnectionOnlySend() { unsafe { WinHttpConnectionOnlySend.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpConnectionSetPolicyEntries")]
pub unsafe extern "C" fn _WinHttpConnectionSetPolicyEntries() { unsafe { WinHttpConnectionSetPolicyEntries.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpConnectionSetProxyInfo")]
pub unsafe extern "C" fn _WinHttpConnectionSetProxyInfo() { unsafe { WinHttpConnectionSetProxyInfo.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpConnectionUpdateIfIndexTable")]
pub unsafe extern "C" fn _WinHttpConnectionUpdateIfIndexTable() { unsafe { WinHttpConnectionUpdateIfIndexTable.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpCrackUrl")]
pub unsafe extern "C" fn _WinHttpCrackUrl() { unsafe { WinHttpCrackUrl.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpCreateProxyList")]
pub unsafe extern "C" fn _WinHttpCreateProxyList() { unsafe { WinHttpCreateProxyList.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpCreateProxyManager")]
pub unsafe extern "C" fn _WinHttpCreateProxyManager() { unsafe { WinHttpCreateProxyManager.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpCreateProxyResolver")]
pub unsafe extern "C" fn _WinHttpCreateProxyResolver() { unsafe { WinHttpCreateProxyResolver.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpCreateProxyResult")]
pub unsafe extern "C" fn _WinHttpCreateProxyResult() { unsafe { WinHttpCreateProxyResult.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpCreateUiCompatibleProxyString")]
pub unsafe extern "C" fn _WinHttpCreateUiCompatibleProxyString() { unsafe { WinHttpCreateUiCompatibleProxyString.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpCreateUrl")]
pub unsafe extern "C" fn _WinHttpCreateUrl() { unsafe { WinHttpCreateUrl.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpDetectAutoProxyConfigUrl")]
pub unsafe extern "C" fn _WinHttpDetectAutoProxyConfigUrl() { unsafe { WinHttpDetectAutoProxyConfigUrl.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpFreeProxyResult")]
pub unsafe extern "C" fn _WinHttpFreeProxyResult() { unsafe { WinHttpFreeProxyResult.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpFreeProxyResultEx")]
pub unsafe extern "C" fn _WinHttpFreeProxyResultEx() { unsafe { WinHttpFreeProxyResultEx.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpFreeProxySettings")]
pub unsafe extern "C" fn _WinHttpFreeProxySettings() { unsafe { WinHttpFreeProxySettings.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpFreeProxySettingsEx")]
pub unsafe extern "C" fn _WinHttpFreeProxySettingsEx() { unsafe { WinHttpFreeProxySettingsEx.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpFreeQueryConnectionGroupResult")]
pub unsafe extern "C" fn _WinHttpFreeQueryConnectionGroupResult() { unsafe { WinHttpFreeQueryConnectionGroupResult.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpGetDefaultProxyConfiguration")]
pub unsafe extern "C" fn _WinHttpGetDefaultProxyConfiguration() { unsafe { WinHttpGetDefaultProxyConfiguration.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpGetIEProxyConfigForCurrentUser")]
pub unsafe extern "C" fn _WinHttpGetIEProxyConfigForCurrentUser() { unsafe { WinHttpGetIEProxyConfigForCurrentUser.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpGetProxyForUrl")]
pub unsafe extern "C" fn _WinHttpGetProxyForUrl() { unsafe { WinHttpGetProxyForUrl.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpGetProxyForUrlEx")]
pub unsafe extern "C" fn _WinHttpGetProxyForUrlEx() { unsafe { WinHttpGetProxyForUrlEx.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpGetProxyForUrlEx2")]
pub unsafe extern "C" fn _WinHttpGetProxyForUrlEx2() { unsafe { WinHttpGetProxyForUrlEx2.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpGetProxyForUrlHvsi")]
pub unsafe extern "C" fn _WinHttpGetProxyForUrlHvsi() { unsafe { WinHttpGetProxyForUrlHvsi.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpGetProxyResult")]
pub unsafe extern "C" fn _WinHttpGetProxyResult() { unsafe { WinHttpGetProxyResult.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpGetProxyResultEx")]
pub unsafe extern "C" fn _WinHttpGetProxyResultEx() { unsafe { WinHttpGetProxyResultEx.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpGetProxySettingsEx")]
pub unsafe extern "C" fn _WinHttpGetProxySettingsEx() { unsafe { WinHttpGetProxySettingsEx.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpGetProxySettingsResultEx")]
pub unsafe extern "C" fn _WinHttpGetProxySettingsResultEx() { unsafe { WinHttpGetProxySettingsResultEx.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpGetProxySettingsVersion")]
pub unsafe extern "C" fn _WinHttpGetProxySettingsVersion() { unsafe { WinHttpGetProxySettingsVersion.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpGetTunnelSocket")]
pub unsafe extern "C" fn _WinHttpGetTunnelSocket() { unsafe { WinHttpGetTunnelSocket.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpOpen")]
pub unsafe extern "C" fn _WinHttpOpen() { unsafe { WinHttpOpen.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpOpenRequest")]
pub unsafe extern "C" fn _WinHttpOpenRequest() { unsafe { WinHttpOpenRequest.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpPacJsWorkerMain")]
pub unsafe extern "C" fn _WinHttpPacJsWorkerMain() { unsafe { WinHttpPacJsWorkerMain.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpProbeConnectivity")]
pub unsafe extern "C" fn _WinHttpProbeConnectivity() { unsafe { WinHttpProbeConnectivity.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpProtocolCompleteUpgrade")]
pub unsafe extern "C" fn _WinHttpProtocolCompleteUpgrade() { unsafe { WinHttpProtocolCompleteUpgrade.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpProtocolReceive")]
pub unsafe extern "C" fn _WinHttpProtocolReceive() { unsafe { WinHttpProtocolReceive.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpProtocolSend")]
pub unsafe extern "C" fn _WinHttpProtocolSend() { unsafe { WinHttpProtocolSend.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpQueryAuthSchemes")]
pub unsafe extern "C" fn _WinHttpQueryAuthSchemes() { unsafe { WinHttpQueryAuthSchemes.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpQueryConnectionGroup")]
pub unsafe extern "C" fn _WinHttpQueryConnectionGroup() { unsafe { WinHttpQueryConnectionGroup.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpQueryDataAvailable")]
pub unsafe extern "C" fn _WinHttpQueryDataAvailable() { unsafe { WinHttpQueryDataAvailable.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpQueryHeaders")]
pub unsafe extern "C" fn _WinHttpQueryHeaders() { unsafe { WinHttpQueryHeaders.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpQueryHeadersEx")]
pub unsafe extern "C" fn _WinHttpQueryHeadersEx() { unsafe { WinHttpQueryHeadersEx.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpQueryOption")]
pub unsafe extern "C" fn _WinHttpQueryOption() { unsafe { WinHttpQueryOption.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpReadData")]
pub unsafe extern "C" fn _WinHttpReadData() { unsafe { WinHttpReadData.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpReadDataEx")]
pub unsafe extern "C" fn _WinHttpReadDataEx() { unsafe { WinHttpReadDataEx.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpReadProxySettings")]
pub unsafe extern "C" fn _WinHttpReadProxySettings() { unsafe { WinHttpReadProxySettings.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpReadProxySettingsHvsi")]
pub unsafe extern "C" fn _WinHttpReadProxySettingsHvsi() { unsafe { WinHttpReadProxySettingsHvsi.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpReceiveResponse")]
pub unsafe extern "C" fn _WinHttpReceiveResponse() { unsafe { WinHttpReceiveResponse.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpRefreshProxySettings")]
pub unsafe extern "C" fn _WinHttpRefreshProxySettings() { unsafe { WinHttpRefreshProxySettings.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpRegisterProxyChangeNotification")]
pub unsafe extern "C" fn _WinHttpRegisterProxyChangeNotification() { unsafe { WinHttpRegisterProxyChangeNotification.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpResetAutoProxy")]
pub unsafe extern "C" fn _WinHttpResetAutoProxy() { unsafe { WinHttpResetAutoProxy.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpResolverGetProxyForUrl")]
pub unsafe extern "C" fn _WinHttpResolverGetProxyForUrl() { unsafe { WinHttpResolverGetProxyForUrl.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpSaveProxyCredentials")]
pub unsafe extern "C" fn _WinHttpSaveProxyCredentials() { unsafe { WinHttpSaveProxyCredentials.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpSendRequest")]
pub unsafe extern "C" fn _WinHttpSendRequest() { unsafe { WinHttpSendRequest.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpSetCredentials")]
pub unsafe extern "C" fn _WinHttpSetCredentials() { unsafe { WinHttpSetCredentials.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpSetDefaultProxyConfiguration")]
pub unsafe extern "C" fn _WinHttpSetDefaultProxyConfiguration() { unsafe { WinHttpSetDefaultProxyConfiguration.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpSetOption")]
pub unsafe extern "C" fn _WinHttpSetOption() { unsafe { WinHttpSetOption.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpSetProxySettingsPerUser")]
pub unsafe extern "C" fn _WinHttpSetProxySettingsPerUser() { unsafe { WinHttpSetProxySettingsPerUser.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpSetSecureLegacyServersAppCompat")]
pub unsafe extern "C" fn _WinHttpSetSecureLegacyServersAppCompat() { unsafe { WinHttpSetSecureLegacyServersAppCompat.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpSetStatusCallback")]
pub unsafe extern "C" fn _WinHttpSetStatusCallback() { unsafe { WinHttpSetStatusCallback.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpSetTimeouts")]
pub unsafe extern "C" fn _WinHttpSetTimeouts() { unsafe { WinHttpSetTimeouts.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpTimeFromSystemTime")]
pub unsafe extern "C" fn _WinHttpTimeFromSystemTime() { unsafe { WinHttpTimeFromSystemTime.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpTimeToSystemTime")]
pub unsafe extern "C" fn _WinHttpTimeToSystemTime() { unsafe { WinHttpTimeToSystemTime.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpUnregisterProxyChangeNotification")]
pub unsafe extern "C" fn _WinHttpUnregisterProxyChangeNotification() { unsafe { WinHttpUnregisterProxyChangeNotification.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpWebSocketClose")]
pub unsafe extern "C" fn _WinHttpWebSocketClose() { unsafe { WinHttpWebSocketClose.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpWebSocketCompleteUpgrade")]
pub unsafe extern "C" fn _WinHttpWebSocketCompleteUpgrade() { unsafe { WinHttpWebSocketCompleteUpgrade.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpWebSocketQueryCloseStatus")]
pub unsafe extern "C" fn _WinHttpWebSocketQueryCloseStatus() { unsafe { WinHttpWebSocketQueryCloseStatus.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpWebSocketReceive")]
pub unsafe extern "C" fn _WinHttpWebSocketReceive() { unsafe { WinHttpWebSocketReceive.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpWebSocketSend")]
pub unsafe extern "C" fn _WinHttpWebSocketSend() { unsafe { WinHttpWebSocketSend.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpWebSocketShutdown")]
pub unsafe extern "C" fn _WinHttpWebSocketShutdown() { unsafe { WinHttpWebSocketShutdown.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpWriteData")]
pub unsafe extern "C" fn _WinHttpWriteData() { unsafe { WinHttpWriteData.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "WinHttpWriteProxySettings")]
pub unsafe extern "C" fn _WinHttpWriteProxySettings() { unsafe { WinHttpWriteProxySettings.unwrap()(); } }

