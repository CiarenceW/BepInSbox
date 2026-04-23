#[repr(C)]
pub enum HostfxrDelegateType
{
    #[allow(non_camel_case_types)]
    HDT_COM_ACTIVATION = 0,
    
    #[allow(non_camel_case_types)]
    HDT_LOAD_IN_MEMORY_ASSEMBLY = 1,
    
    #[allow(non_camel_case_types)]
    HDT_WINRT_ACTIVATION = 2,
    
    #[allow(non_camel_case_types)]
    HDT_COM_REGISTER = 3,
    
    #[allow(non_camel_case_types)]
    HDT_COM_UNREGISTER = 4,
    
    #[allow(non_camel_case_types)]
    HDT_LOAD_ASSEMBLY_AND_GET_FUNCTION_POINTER = 5,
    
    #[allow(non_camel_case_types)]
    HDT_GET_FUNCTION_POINTER = 6,
    
    #[allow(non_camel_case_types)]
    HDT_LOAD_ASSEMBLY = 7,
    
    #[allow(non_camel_case_types)]
    HDT_LOAD_ASSEMBLY_BYTE = 8,
}

#[cfg(target_family = "windows")]
#[allow(non_camel_case_types)]
pub type char_t = std::primitive::u16;

#[cfg(not(target_family = "windows"))]
#[allow(non_camel_case_types)]
pub type char_t = std::primitive::u8;

pub type HostfxrMainFn = 
	unsafe extern "C" fn(argc: ::std::ffi::c_int, argv: *mut *const char_t) -> i32;

pub type HostfxrMainStartupinfoFn =
    unsafe extern "C" fn(
        argc: ::std::ffi::c_int,
        argv: *mut *const char_t,
        host_path: *const char_t,
        dotnet_root: *const char_t,
        app_path: *const char_t,
    ) -> i32;

pub type HostfxrMainBundleStartupinfoFn =
    unsafe extern "C" fn(
        argc: ::std::ffi::c_int,
        argv: *mut *const char_t,
        host_path: *const char_t,
        dotnet_root: *const char_t,
        app_path: *const char_t,
        bundle_header_offset: i64,
    ) -> i32;

pub type HostfxrErrorWriterFn =
    unsafe extern "C" fn(message: *const char_t);

//
// Sets a callback which is to be used to write errors to.
//
// Parameters:
//     error_writer
//         A callback function which will be invoked every time an error is to be reported.
//         Or nullptr to unregister previously registered callback and return to the default behavior.
// Return value:
//     The previously registered callback (which is now unregistered), or nullptr if no previous callback
//     was registered
//
// The error writer is registered per-thread, so the registration is thread-local. On each thread
// only one callback can be registered. Subsequent registrations overwrite the previous ones.
//
// By default no callback is registered in which case the errors are written to stderr.
//
// Each call to the error writer is sort of like writing a single line (the EOL character is omitted).
// Multiple calls to the error writer may occur for one failure.
//
// If the hostfxr invokes functions in hostpolicy as part of its operation, the error writer
// will be propagated to hostpolicy for the duration of the call. This means that errors from
// both hostfxr and hostpolicy will be reporter through the same error writer.
//
pub type HostfxrSetErrorWriterFn = 
    unsafe extern "C" fn(error_writer: HostfxrErrorWriterFn) -> HostfxrErrorWriterFn;

pub type HostfxrHandle = *const std::ffi::c_void;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hostfxr_initialize_parameters {
    pub size: usize,
    pub host_path: *const char_t,
    pub dotnet_root: *const char_t,
}

//
// Initializes the hosting components for a dotnet command line running an application
//
// Parameters:
//    argc
//      Number of argv arguments
//    argv
//      Command-line arguments for running an application (as if through the dotnet executable).
//      Only command-line arguments which are accepted by runtime installation are supported, SDK/CLI commands are not supported.
//      For example 'app.dll app_argument_1 app_argument_2`.
//    parameters
//      Optional. Additional parameters for initialization
//    host_context_handle
//      On success, this will be populated with an opaque value representing the initialized host context
//
// Return value:
//    Success          - Hosting components were successfully initialized
//    HostInvalidState - Hosting components are already initialized
//
// This function parses the specified command-line arguments to determine the application to run. It will
// then find the corresponding .runtimeconfig.json and .deps.json with which to resolve frameworks and
// dependencies and prepare everything needed to load the runtime.
//
// This function only supports arguments for running an application. It does not support SDK commands.
//
// This function does not load the runtime.
//
pub type HostfxrInitializeForDotnetCommandLineFn = 
    unsafe extern "C" fn(
        argc: ::std::ffi::c_int,
        argv: *mut *const char_t,
        parameters: *const hostfxr_initialize_parameters,
        host_context_handle: *mut HostfxrHandle,
    ) -> i32;

//
// Initializes the hosting components using a .runtimeconfig.json file
//
// Parameters:
//    runtime_config_path
//      Path to the .runtimeconfig.json file
//    parameters
//      Optional. Additional parameters for initialization
//    host_context_handle
//      On success, this will be populated with an opaque value representing the initialized host context
//
// Return value:
//    Success                            - Hosting components were successfully initialized
//    Success_HostAlreadyInitialized     - Config is compatible with already initialized hosting components
//    Success_DifferentRuntimeProperties - Config has runtime properties that differ from already initialized hosting components
//    HostIncompatibleConfig             - Config is incompatible with already initialized hosting components
//
// This function will process the .runtimeconfig.json to resolve frameworks and prepare everything needed
// to load the runtime. It will only process the .deps.json from frameworks (not any app/component that
// may be next to the .runtimeconfig.json).
//
// This function does not load the runtime.
//
// If called when the runtime has already been loaded, this function will check if the specified runtime
// config is compatible with the existing runtime.
//
// Both Success_HostAlreadyInitialized and Success_DifferentRuntimeProperties codes are considered successful
// initializations. In the case of Success_DifferentRuntimeProperties, it is left to the consumer to verify that
// the difference in properties is acceptable.
//
pub type HostfxrInitializeForRuntimeConfigFn =
    unsafe extern "C" fn(
        runtime_config_path: *const char_t,
        parameters: *const hostfxr_initialize_parameters,
        host_context_handle: *mut HostfxrHandle,
    ) -> i32;

//
// Gets the runtime property value for an initialized host context
//
// Parameters:
//     host_context_handle
//       Handle to the initialized host context
//     name
//       Runtime property name
//     value
//       Out parameter. Pointer to a buffer with the property value.
//
// Return value:
//     The error code result.
//
// The buffer pointed to by value is owned by the host context. The lifetime of the buffer is only
// guaranteed until any of the below occur:
//   - a 'run' method is called for the host context
//   - properties are changed via hostfxr_set_runtime_property_value
//   - the host context is closed via 'hostfxr_close'
//
// If host_context_handle is nullptr and an active host context exists, this function will get the
// property value for the active host context.
//
pub type HostfxrGetRuntimePropertyValueFn =
    unsafe extern "C" fn(
        host_context_handle: HostfxrHandle,
        name: *const char_t,
        value: *mut *const char_t,
    ) -> i32;

//
// Sets the value of a runtime property for an initialized host context
//
// Parameters:
//     host_context_handle
//       Handle to the initialized host context
//     name
//       Runtime property name
//     value
//       Value to set
//
// Return value:
//     The error code result.
//
// Setting properties is only supported for the first host context, before the runtime has been loaded.
//
// If the property already exists in the host context, it will be overwritten. If value is nullptr, the
// property will be removed.
//
pub type HostfxrSetRuntimePropertyValueFn =
    unsafe extern "C" fn(
        host_context_handle: HostfxrHandle,
        name: *const char_t,
        value: *const char_t,
    ) -> i32;

//
// Gets all the runtime properties for an initialized host context
//
// Parameters:
//     host_context_handle
//       Handle to the initialized host context
//     count
//       [in] Size of the keys and values buffers
//       [out] Number of properties returned (size of keys/values buffers used). If the input value is too
//             small or keys/values is nullptr, this is populated with the number of available properties
//     keys
//       Array of pointers to buffers with runtime property keys
//     values
//       Array of pointers to buffers with runtime property values
//
// Return value:
//     The error code result.
//
// The buffers pointed to by keys and values are owned by the host context. The lifetime of the buffers is only
// guaranteed until any of the below occur:
//   - a 'run' method is called for the host context
//   - properties are changed via hostfxr_set_runtime_property_value
//   - the host context is closed via 'hostfxr_close'
//
// If host_context_handle is nullptr and an active host context exists, this function will get the
// properties for the active host context.
//
pub type HostfxrGetRuntimePropertiesFn =
    unsafe extern "C" fn(
        host_context_handle: HostfxrHandle,
        count: *mut usize,
        keys: *mut *const char_t,
        values: *mut *const char_t,
    ) -> i32;

//
// Load CoreCLR and run the application for an initialized host context
//
// Parameters:
//     host_context_handle
//       Handle to the initialized host context
//
// Return value:
//     If the app was successfully run, the exit code of the application. Otherwise, the error code result.
//
// The host_context_handle must have been initialized using hostfxr_initialize_for_dotnet_command_line.
//
// This function will not return until the managed application exits.
//
pub type HostfxrRunAppFn =
	unsafe extern "C" fn(host_context_handle: HostfxrHandle) -> i32;

//
// Gets a typed delegate from the currently loaded CoreCLR or from a newly created one.
//
// Parameters:
//     host_context_handle
//       Handle to the initialized host context
//     type
//       Type of runtime delegate requested
//     delegate
//       An out parameter that will be assigned the delegate.
//
// Return value:
//     The error code result.
//
// If the host_context_handle was initialized using hostfxr_initialize_for_runtime_config,
// then all delegate types are supported.
// If the host_context_handle was initialized using hostfxr_initialize_for_dotnet_command_line,
// then only the following delegate types are currently supported:
//     hdt_load_assembly_and_get_function_pointer
//     hdt_get_function_pointer
//
pub type HostfxrGetRuntimeDelegateFn =
    unsafe extern "C" fn(
        host_context_handle: HostfxrHandle,
        type_: HostfxrDelegateType,
        delegate: *mut *mut ::std::ffi::c_void,
    ) -> std::ffi::c_int;

//
// Closes an initialized host context
//
// Parameters:
//     host_context_handle
//       Handle to the initialized host context
//
// Return value:
//     The error code result.
//
pub type HostfxrCloseFn =
    unsafe extern "C" fn(host_context_handle: HostfxrHandle) -> i32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hostfxr_dotnet_environment_sdk_info {
    pub size: usize,
    pub version: *const char_t,
    pub path: *const char_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hostfxr_dotnet_environment_framework_info {
    pub size: usize,
    pub name: *const char_t,
    pub version: *const char_t,
    pub path: *const char_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hostfxr_dotnet_environment_info {
    pub size: usize,
    pub hostfxr_version: *const char_t,
    pub hostfxr_commit_hash: *const char_t,
    pub sdk_count: usize,
    pub sdks: *const hostfxr_dotnet_environment_sdk_info,
    pub framework_count: usize,
    pub frameworks: *const hostfxr_dotnet_environment_framework_info,
}

pub type HostfxrGetDotnetEnvironmentInfoResultFn =
    unsafe extern "C" fn(
        info: *const hostfxr_dotnet_environment_info,
        result_context: *mut ::std::ffi::c_void,
    );

//
// Returns available SDKs and frameworks.
//
// Resolves the existing SDKs and frameworks from a dotnet root directory (if
// any), or the global default location. If multi-level lookup is enabled and
// the dotnet root location is different than the global location, the SDKs and
// frameworks will be enumerated from both locations.
//
// The SDKs are sorted in ascending order by version, multi-level lookup
// locations are put before private ones.
//
// The frameworks are sorted in ascending order by name followed by version,
// multi-level lookup locations are put before private ones.
//
// Parameters:
//    dotnet_root
//      The path to a directory containing a dotnet executable.
//
//    reserved
//      Reserved for future parameters.
//
//    result
//      Callback invoke to return the list of SDKs and frameworks.
//      Structs and their elements are valid for the duration of the call.
//
//    result_context
//      Additional context passed to the result callback.
//
// Return value:
//   0 on success, otherwise failure.
//
// String encoding:
//   Windows     - UTF-16 (pal::char_t is 2 byte wchar_t)
//   Non-Windows - UTF-8  (pal::char_t is 1 byte char)
//
pub type HostfxrGetDotnetEnvironmentInfoFn =
    unsafe extern "C" fn(
        dotnet_root: *const char_t,
        reserved: *mut ::std::ffi::c_void,
        result: HostfxrGetDotnetEnvironmentInfoResultFn,
        result_context: *mut ::std::ffi::c_void,
    ) -> i32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hostfxr_framework_result {
    pub size: usize,
    pub name: *const char_t,
    pub requested_version: *const char_t,
    pub resolved_version: *const char_t,
    pub resolved_path: *const char_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hostfxr_resolve_frameworks_result {
    pub size: usize,
    pub resolved_count: usize,
    pub resolved_frameworks: *const hostfxr_framework_result,
    pub unresolved_count: usize,
    pub unresolved_frameworks: *const hostfxr_framework_result,
}

pub type HostfxrResolveFrameworksResultFn =
    unsafe extern "C" fn(
        result: *const hostfxr_resolve_frameworks_result,
        result_context: *mut ::std::ffi::c_void,
    );

//
// Resolves frameworks for a runtime config
//
// Parameters:
//    runtime_config_path
//      Path to the .runtimeconfig.json file
//    parameters
//      Optional. Additional parameters for initialization.
//      If null or dotnet_root is null, the root corresponding to the running hostfx is used.
//    callback
//      Optional. Result callback invoked with result of the resolution.
//      Structs and their elements are valid for the duration of the call.
//    result_context
//      Optional. Additional context passed to the result callback.
//
// Return value:
//   0 on success, otherwise failure.
//
// String encoding:
//   Windows     - UTF-16 (pal::char_t is 2-byte wchar_t)
//   Non-Windows - UTF-8  (pal::char_t is 1-byte char)
//
pub type HostfxrResolveFrameworksForRuntimeConfigFn =
    unsafe extern "C" fn(
        runtime_config_path: *const char_t,
        parameters: *const hostfxr_initialize_parameters,
        callback: HostfxrResolveFrameworksResultFn,
        result_context: *mut ::std::ffi::c_void,
    ) -> i32;
