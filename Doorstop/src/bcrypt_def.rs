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
static mut BCryptAddContextFunction: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptAddContextFunctionProvider: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptCloseAlgorithmProvider: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptConfigureContext: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptConfigureContextFunction: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptCreateContext: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptCreateHash: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptCreateMultiHash: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptDecapsulate: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptDecrypt: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptDeleteContext: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptDeriveKey: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptDeriveKeyCapi: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptDeriveKeyPBKDF2: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptDestroyHash: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptDestroyKey: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptDestroySecret: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptDuplicateHash: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptDuplicateKey: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptEncapsulate: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptEncrypt: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptEnumAlgorithms: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptEnumContextFunctionProviders: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptEnumContextFunctions: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptEnumContexts: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptEnumProviders: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptEnumRegisteredProviders: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptExportKey: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptFinalizeKeyPair: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptFinishHash: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptFreeBuffer: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptGenRandom: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptGenerateKeyPair: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptGenerateSymmetricKey: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptGetFipsAlgorithmMode: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptGetProperty: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptHash: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptHashData: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptImportKey: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptImportKeyPair: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptKeyDerivation: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptOpenAlgorithmProvider: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptProcessMultiOperations: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptQueryContextConfiguration: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptQueryContextFunctionConfiguration: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptQueryContextFunctionProperty: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptQueryProviderRegistration: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptRegisterConfigChangeNotify: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptRegisterProvider: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptRemoveContextFunction: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptRemoveContextFunctionProvider: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptResolveProviders: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptSecretAgreement: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptSetAuditingInterface: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptSetContextFunctionProperty: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptSetProperty: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptSignHash: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptUnregisterConfigChangeNotify: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptUnregisterProvider: FARPROC = None;

#[allow(non_upper_case_globals)]
static mut BCryptVerifySignature: FARPROC = None;

#[cfg(target_family = "windows")]
#[allow(non_snake_case)]
pub fn LoadOriginalLibrary(module: HMODULE)
{
    unsafe
    {
        dll = module;
        BCryptAddContextFunction = GetProcAddress(dll, s!("BCryptAddContextFunction"));
        BCryptAddContextFunctionProvider = GetProcAddress(dll, s!("BCryptAddContextFunctionProvider"));
        BCryptCloseAlgorithmProvider = GetProcAddress(dll, s!("BCryptCloseAlgorithmProvider"));
        BCryptConfigureContext = GetProcAddress(dll, s!("BCryptConfigureContext"));
        BCryptConfigureContextFunction = GetProcAddress(dll, s!("BCryptConfigureContextFunction"));
        BCryptCreateContext = GetProcAddress(dll, s!("BCryptCreateContext"));
        BCryptCreateHash = GetProcAddress(dll, s!("BCryptCreateHash"));
        BCryptCreateMultiHash = GetProcAddress(dll, s!("BCryptCreateMultiHash"));
        BCryptDecapsulate = GetProcAddress(dll, s!("BCryptDecapsulate"));
        BCryptDecrypt = GetProcAddress(dll, s!("BCryptDecrypt"));
        BCryptDeleteContext = GetProcAddress(dll, s!("BCryptDeleteContext"));
        BCryptDeriveKey = GetProcAddress(dll, s!("BCryptDeriveKey"));
        BCryptDeriveKeyCapi = GetProcAddress(dll, s!("BCryptDeriveKeyCapi"));
        BCryptDeriveKeyPBKDF2 = GetProcAddress(dll, s!("BCryptDeriveKeyPBKDF2"));
        BCryptDestroyHash = GetProcAddress(dll, s!("BCryptDestroyHash"));
        BCryptDestroyKey = GetProcAddress(dll, s!("BCryptDestroyKey"));
        BCryptDestroySecret = GetProcAddress(dll, s!("BCryptDestroySecret"));
        BCryptDuplicateHash = GetProcAddress(dll, s!("BCryptDuplicateHash"));
        BCryptDuplicateKey = GetProcAddress(dll, s!("BCryptDuplicateKey"));
        BCryptEncapsulate = GetProcAddress(dll, s!("BCryptEncapsulate"));
        BCryptEncrypt = GetProcAddress(dll, s!("BCryptEncrypt"));
        BCryptEnumAlgorithms = GetProcAddress(dll, s!("BCryptEnumAlgorithms"));
        BCryptEnumContextFunctionProviders = GetProcAddress(dll, s!("BCryptEnumContextFunctionProviders"));
        BCryptEnumContextFunctions = GetProcAddress(dll, s!("BCryptEnumContextFunctions"));
        BCryptEnumContexts = GetProcAddress(dll, s!("BCryptEnumContexts"));
        BCryptEnumProviders = GetProcAddress(dll, s!("BCryptEnumProviders"));
        BCryptEnumRegisteredProviders = GetProcAddress(dll, s!("BCryptEnumRegisteredProviders"));
        BCryptExportKey = GetProcAddress(dll, s!("BCryptExportKey"));
        BCryptFinalizeKeyPair = GetProcAddress(dll, s!("BCryptFinalizeKeyPair"));
        BCryptFinishHash = GetProcAddress(dll, s!("BCryptFinishHash"));
        BCryptFreeBuffer = GetProcAddress(dll, s!("BCryptFreeBuffer"));
        BCryptGenRandom = GetProcAddress(dll, s!("BCryptGenRandom"));
        BCryptGenerateKeyPair = GetProcAddress(dll, s!("BCryptGenerateKeyPair"));
        BCryptGenerateSymmetricKey = GetProcAddress(dll, s!("BCryptGenerateSymmetricKey"));
        BCryptGetFipsAlgorithmMode = GetProcAddress(dll, s!("BCryptGetFipsAlgorithmMode"));
        BCryptGetProperty = GetProcAddress(dll, s!("BCryptGetProperty"));
        BCryptHash = GetProcAddress(dll, s!("BCryptHash"));
        BCryptHashData = GetProcAddress(dll, s!("BCryptHashData"));
        BCryptImportKey = GetProcAddress(dll, s!("BCryptImportKey"));
        BCryptImportKeyPair = GetProcAddress(dll, s!("BCryptImportKeyPair"));
        BCryptKeyDerivation = GetProcAddress(dll, s!("BCryptKeyDerivation"));
        BCryptOpenAlgorithmProvider = GetProcAddress(dll, s!("BCryptOpenAlgorithmProvider"));
        BCryptProcessMultiOperations = GetProcAddress(dll, s!("BCryptProcessMultiOperations"));
        BCryptQueryContextConfiguration = GetProcAddress(dll, s!("BCryptQueryContextConfiguration"));
        BCryptQueryContextFunctionConfiguration = GetProcAddress(dll, s!("BCryptQueryContextFunctionConfiguration"));
        BCryptQueryContextFunctionProperty = GetProcAddress(dll, s!("BCryptQueryContextFunctionProperty"));
        BCryptQueryProviderRegistration = GetProcAddress(dll, s!("BCryptQueryProviderRegistration"));
        BCryptRegisterConfigChangeNotify = GetProcAddress(dll, s!("BCryptRegisterConfigChangeNotify"));
        BCryptRegisterProvider = GetProcAddress(dll, s!("BCryptRegisterProvider"));
        BCryptRemoveContextFunction = GetProcAddress(dll, s!("BCryptRemoveContextFunction"));
        BCryptRemoveContextFunctionProvider = GetProcAddress(dll, s!("BCryptRemoveContextFunctionProvider"));
        BCryptResolveProviders = GetProcAddress(dll, s!("BCryptResolveProviders"));
        BCryptSecretAgreement = GetProcAddress(dll, s!("BCryptSecretAgreement"));
        BCryptSetAuditingInterface = GetProcAddress(dll, s!("BCryptSetAuditingInterface"));
        BCryptSetContextFunctionProperty = GetProcAddress(dll, s!("BCryptSetContextFunctionProperty"));
        BCryptSetProperty = GetProcAddress(dll, s!("BCryptSetProperty"));
        BCryptSignHash = GetProcAddress(dll, s!("BCryptSignHash"));
        BCryptUnregisterConfigChangeNotify = GetProcAddress(dll, s!("BCryptUnregisterConfigChangeNotify"));
        BCryptUnregisterProvider = GetProcAddress(dll, s!("BCryptUnregisterProvider"));
        BCryptVerifySignature = GetProcAddress(dll, s!("BCryptVerifySignature"));
    }
}

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptAddContextFunction")]
pub extern "C" fn _BCryptAddContextFunction() { unsafe { BCryptAddContextFunction.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptAddContextFunctionProvider")]
pub extern "C" fn _BCryptAddContextFunctionProvider() { unsafe { BCryptAddContextFunctionProvider.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptCloseAlgorithmProvider")]
pub extern "C" fn _BCryptCloseAlgorithmProvider() { unsafe { BCryptCloseAlgorithmProvider.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptConfigureContext")]
pub extern "C" fn _BCryptConfigureContext() { unsafe { BCryptConfigureContext.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptConfigureContextFunction")]
pub extern "C" fn _BCryptConfigureContextFunction() { unsafe { BCryptConfigureContextFunction.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptCreateContext")]
pub extern "C" fn _BCryptCreateContext() { unsafe { BCryptCreateContext.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptCreateHash")]
pub extern "C" fn _BCryptCreateHash() { unsafe { BCryptCreateHash.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptCreateMultiHash")]
pub extern "C" fn _BCryptCreateMultiHash() { unsafe { BCryptCreateMultiHash.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptDecapsulate")]
pub extern "C" fn _BCryptDecapsulate() { unsafe { BCryptDecapsulate.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptDecrypt")]
pub extern "C" fn _BCryptDecrypt() { unsafe { BCryptDecrypt.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptDeleteContext")]
pub extern "C" fn _BCryptDeleteContext() { unsafe { BCryptDeleteContext.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptDeriveKey")]
pub extern "C" fn _BCryptDeriveKey() { unsafe { BCryptDeriveKey.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptDeriveKeyCapi")]
pub extern "C" fn _BCryptDeriveKeyCapi() { unsafe { BCryptDeriveKeyCapi.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptDeriveKeyPBKDF2")]
pub extern "C" fn _BCryptDeriveKeyPBKDF2() { unsafe { BCryptDeriveKeyPBKDF2.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptDestroyHash")]
pub extern "C" fn _BCryptDestroyHash() { unsafe { BCryptDestroyHash.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptDestroyKey")]
pub extern "C" fn _BCryptDestroyKey() { unsafe { BCryptDestroyKey.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptDestroySecret")]
pub extern "C" fn _BCryptDestroySecret() { unsafe { BCryptDestroySecret.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptDuplicateHash")]
pub extern "C" fn _BCryptDuplicateHash() { unsafe { BCryptDuplicateHash.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptDuplicateKey")]
pub extern "C" fn _BCryptDuplicateKey() { unsafe { BCryptDuplicateKey.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptEncapsulate")]
pub extern "C" fn _BCryptEncapsulate() { unsafe { BCryptEncapsulate.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptEncrypt")]
pub extern "C" fn _BCryptEncrypt() { unsafe { BCryptEncrypt.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptEnumAlgorithms")]
pub extern "C" fn _BCryptEnumAlgorithms() { unsafe { BCryptEnumAlgorithms.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptEnumContextFunctionProviders")]
pub extern "C" fn _BCryptEnumContextFunctionProviders() { unsafe { BCryptEnumContextFunctionProviders.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptEnumContextFunctions")]
pub extern "C" fn _BCryptEnumContextFunctions() { unsafe { BCryptEnumContextFunctions.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptEnumContexts")]
pub extern "C" fn _BCryptEnumContexts() { unsafe { BCryptEnumContexts.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptEnumProviders")]
pub extern "C" fn _BCryptEnumProviders() { unsafe { BCryptEnumProviders.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptEnumRegisteredProviders")]
pub extern "C" fn _BCryptEnumRegisteredProviders() { unsafe { BCryptEnumRegisteredProviders.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptExportKey")]
pub extern "C" fn _BCryptExportKey() { unsafe { BCryptExportKey.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptFinalizeKeyPair")]
pub extern "C" fn _BCryptFinalizeKeyPair() { unsafe { BCryptFinalizeKeyPair.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptFinishHash")]
pub extern "C" fn _BCryptFinishHash() { unsafe { BCryptFinishHash.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptFreeBuffer")]
pub extern "C" fn _BCryptFreeBuffer() { unsafe { BCryptFreeBuffer.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptGenRandom")]
pub extern "C" fn _BCryptGenRandom() { unsafe { BCryptGenRandom.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptGenerateKeyPair")]
pub extern "C" fn _BCryptGenerateKeyPair() { unsafe { BCryptGenerateKeyPair.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptGenerateSymmetricKey")]
pub extern "C" fn _BCryptGenerateSymmetricKey() { unsafe { BCryptGenerateSymmetricKey.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptGetFipsAlgorithmMode")]
pub extern "C" fn _BCryptGetFipsAlgorithmMode() { unsafe { BCryptGetFipsAlgorithmMode.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptGetProperty")]
pub extern "C" fn _BCryptGetProperty() { unsafe { BCryptGetProperty.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptHash")]
pub extern "C" fn _BCryptHash() { unsafe { BCryptHash.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptHashData")]
pub extern "C" fn _BCryptHashData() { unsafe { BCryptHashData.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptImportKey")]
pub extern "C" fn _BCryptImportKey() { unsafe { BCryptImportKey.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptImportKeyPair")]
pub extern "C" fn _BCryptImportKeyPair() { unsafe { BCryptImportKeyPair.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptKeyDerivation")]
pub extern "C" fn _BCryptKeyDerivation() { unsafe { BCryptKeyDerivation.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptOpenAlgorithmProvider")]
pub extern "C" fn _BCryptOpenAlgorithmProvider() { unsafe { BCryptOpenAlgorithmProvider.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptProcessMultiOperations")]
pub extern "C" fn _BCryptProcessMultiOperations() { unsafe { BCryptProcessMultiOperations.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptQueryContextConfiguration")]
pub extern "C" fn _BCryptQueryContextConfiguration() { unsafe { BCryptQueryContextConfiguration.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptQueryContextFunctionConfiguration")]
pub extern "C" fn _BCryptQueryContextFunctionConfiguration() { unsafe { BCryptQueryContextFunctionConfiguration.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptQueryContextFunctionProperty")]
pub extern "C" fn _BCryptQueryContextFunctionProperty() { unsafe { BCryptQueryContextFunctionProperty.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptQueryProviderRegistration")]
pub extern "C" fn _BCryptQueryProviderRegistration() { unsafe { BCryptQueryProviderRegistration.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptRegisterConfigChangeNotify")]
pub extern "C" fn _BCryptRegisterConfigChangeNotify() { unsafe { BCryptRegisterConfigChangeNotify.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptRegisterProvider")]
pub extern "C" fn _BCryptRegisterProvider() { unsafe { BCryptRegisterProvider.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptRemoveContextFunction")]
pub extern "C" fn _BCryptRemoveContextFunction() { unsafe { BCryptRemoveContextFunction.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptRemoveContextFunctionProvider")]
pub extern "C" fn _BCryptRemoveContextFunctionProvider() { unsafe { BCryptRemoveContextFunctionProvider.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptResolveProviders")]
pub extern "C" fn _BCryptResolveProviders() { unsafe { BCryptResolveProviders.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptSecretAgreement")]
pub extern "C" fn _BCryptSecretAgreement() { unsafe { BCryptSecretAgreement.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptSetAuditingInterface")]
pub extern "C" fn _BCryptSetAuditingInterface() { unsafe { BCryptSetAuditingInterface.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptSetContextFunctionProperty")]
pub extern "C" fn _BCryptSetContextFunctionProperty() { unsafe { BCryptSetContextFunctionProperty.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptSetProperty")]
pub extern "C" fn _BCryptSetProperty() { unsafe { BCryptSetProperty.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptSignHash")]
pub extern "C" fn _BCryptSignHash() { unsafe { BCryptSignHash.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptUnregisterConfigChangeNotify")]
pub extern "C" fn _BCryptUnregisterConfigChangeNotify() { unsafe { BCryptUnregisterConfigChangeNotify.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptUnregisterProvider")]
pub extern "C" fn _BCryptUnregisterProvider() { unsafe { BCryptUnregisterProvider.unwrap()(); } }

#[unsafe(no_mangle)]
#[unsafe(export_name = "BCryptVerifySignature")]
pub extern "C" fn _BCryptVerifySignature() { unsafe { BCryptVerifySignature.unwrap()(); } }
