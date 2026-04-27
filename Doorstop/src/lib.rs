#[cfg(target_family = "windows")]
use std::io::Bytes;
#[cfg(not(target_family = "windows"))]
use std::os::unix::ffi::OsStrExt;

use std::{char::MAX, env, ffi::{OsStr, OsString}, fs::File, io::Write, ffi::c_void, path::{Path, PathBuf}, str::FromStr, time::SystemTime};

use ctor::ctor;

mod lib { pub mod hostfxr; pub mod coreclr_delegates; pub mod nethost; }

#[cfg(target_family = "windows")]
mod xinput_def;

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

use libloading::*;

use crate::lib::{coreclr_delegates::{ComponentEntryPointFn, LoadAssemblyAndGetFunctionPointerFn}, hostfxr::{self, HostfxrCloseFn, HostfxrGetRuntimeDelegateFn, HostfxrHandle, HostfxrInitializeForRuntimeConfigFn}};

#[cfg(target_family = "windows")]
#[allow(non_camel_case_types)]
pub type char_t = u16;

#[cfg(target_family = "unix")]
#[allow(non_camel_case_types)]
pub type char_t = u8;

#[cfg(target_family = "windows")]
const DEBUG_WITH_MESSAGE_BOXES: bool = false;

#[cfg(target_family = "windows")]
const DLL_NAME: &'static str = "XInput1_4";

#[cfg(target_family = "unix")]
const DLL_NAME: &'static str = "RustyDoorstop";

#[cfg(not(target_family = "windows"))]
const MAX_PATH: usize = 255;

struct DoorstopBootSettings
{
	enabled: bool,
	target_assembly_path: PathBuf,
	target_type_namespace: String,
	target_method_name: String,
	runtime_config_name: String
}

fn get_dotnet_load_assembly(config_path: OsString, init_for_config_fptr: HostfxrInitializeForRuntimeConfigFn, get_delegate_fptr: HostfxrGetRuntimeDelegateFn, close_fptr: HostfxrCloseFn, mut out_file: &File) -> LoadAssemblyAndGetFunctionPointerFn
{
	unsafe
	{
		let mut load_assembly_and_get_function_pointer: *mut c_void = std::ptr::null_mut();

		let rc = get_delegate_fptr(std::ptr::null(), crate::lib::hostfxr::HostfxrDelegateType::HDT_LOAD_ASSEMBLY_AND_GET_FUNCTION_POINTER, &mut load_assembly_and_get_function_pointer);

		if (rc >= 0)
		{
			out_file.write_all(b"skipping initialising host, there's already one running\n");

			return std::mem::transmute(load_assembly_and_get_function_pointer);
		}

        out_file.write_all(format!("host doesn't exist, initialising a new one, lol: {rc:x}\n").as_bytes());

		let mut cxt: HostfxrHandle = std::ptr::null_mut();

		#[cfg(target_family = "windows")]
		//warning!!! this doesn't actually work lol, it fucks up for some reason and a bunch of bullshit gets added at the end lol :)
		let mut path_bytes: Vec<u16> = config_path.to_str().unwrap().encode_utf16().collect();

        #[cfg(target_family = "windows")]
        path_bytes.push(0);

		#[cfg(not(target_family = "windows"))]
		let path_bytes = config_path.as_bytes();

        out_file.write_all(format!("full path byte array: {:?}\n", path_bytes).as_bytes());
		
		let rc = init_for_config_fptr(path_bytes.as_ptr(), std::ptr::null(), &mut cxt);

		if (cxt.is_null())
		{
			out_file.write_all(format!("Init failed: {:x}\n", rc).as_bytes());

			close_fptr(cxt);

			return std::mem::transmute(std::ptr::null::<LoadAssemblyAndGetFunctionPointerFn>());
		}

		out_file.write_all(format!("HostFXR Content Handle: {:x}\n", cxt as usize).as_bytes());

		let rc = get_delegate_fptr(cxt, crate::lib::hostfxr::HostfxrDelegateType::HDT_LOAD_ASSEMBLY_AND_GET_FUNCTION_POINTER, &mut load_assembly_and_get_function_pointer);

		if (rc != 0 || load_assembly_and_get_function_pointer.is_null())
		{
			out_file.write_all(format!("Get delegate failed: {:x}", rc).as_bytes());
		}

		return std::mem::transmute(std::ptr::null::<LoadAssemblyAndGetFunctionPointerFn>());
	}
}

fn load_library() -> Library
{
	unsafe
	{
		let mut buffer: [char_t; MAX_PATH as usize] = [0; MAX_PATH as usize];
		let mut buffer_size: usize = size_of_val(&buffer) / size_of::<char_t>();
		
		let rc = crate::lib::nethost::get_hostfxr_path(buffer.as_mut_ptr(), &mut buffer_size, std::ptr::null());

		let filled_buffer = &buffer[..buffer_size];
		
		if (rc != 0)
		{
			!panic!("couldn't find hostfxr_path fuckkkkkkkkkkkkkkkkkkkkkkkkkk");
		}

		#[cfg(target_family = "windows")]
		let hostfxr_path = String::from_utf16(&filled_buffer).expect("Failed to convert buffer to string :(");

		#[cfg(not(target_family = "windows"))]
		let hostfxr_path = String::from_utf8(filled_buffer.to_vec()).expect("Failed to convert buffer to string :(");

		return libloading::Library::new(hostfxr_path).expect("failed to load hostfxr library from path");
	}
}

fn get_export<'a, T>(lib: &'a Library, symbol: &'static str) -> Symbol<'a , T>
{
	unsafe 
	{
		let symbol = lib.get(symbol).expect("Failed to load symbol");

		return symbol;
	}
}

fn load_hostfxr<'delegate_life>(lib: &'delegate_life Library) -> (Symbol<'delegate_life, HostfxrInitializeForRuntimeConfigFn>, Symbol<'delegate_life, HostfxrGetRuntimeDelegateFn>, Symbol<'delegate_life, HostfxrCloseFn>)
{
	unsafe
	{
		let init_for_config_fptr = get_export::<crate::lib::hostfxr::HostfxrInitializeForRuntimeConfigFn>(&lib, "hostfxr_initialize_for_runtime_config");
		let get_delegate_fptr = get_export::<crate::lib::hostfxr::HostfxrGetRuntimeDelegateFn>(&lib, "hostfxr_get_runtime_delegate");
		let close_fptr = get_export::<crate::lib::hostfxr::HostfxrCloseFn>(&lib, "hostfxr_close");
				
		return (init_for_config_fptr, get_delegate_fptr, close_fptr);
		
	}
}

fn load_entry_point_method(load_assembly_and_get_function_pointer: LoadAssemblyAndGetFunctionPointerFn, mut out_file: &File, settings: DoorstopBootSettings)
{
	unsafe
	{
        let path = settings.target_assembly_path;

		#[cfg(target_family = "windows")]
		let mut entrypoint_dll_path: Vec<u16> = path.to_str().unwrap().encode_utf16().collect();

        #[cfg(target_family = "windows")]
        entrypoint_dll_path.push(0);

		#[cfg(not(target_family = "windows"))]
		let entrypoint_dll_path = path.clone().into_os_string().as_bytes().to_owned();

        out_file.write_all(format!("BepInSbox entrypoint dll: {}\n", path.display()).as_bytes());
		
		let dotnet_type = settings.target_type_namespace;

		#[cfg(target_family = "windows")]
		let mut dotnet_type: Vec<u16> = dotnet_type.encode_utf16().collect();

        #[cfg(target_family = "windows")]
        dotnet_type.push(0);

		let dotnet_type_method = settings.target_method_name;

		#[cfg(target_family = "windows")]
		let mut dotnet_type_method: Vec<u16> = dotnet_type_method.encode_utf16().collect();

        #[cfg(target_family = "windows")]
        dotnet_type_method.push(0);
		
		let mut bootstrap: *mut ComponentEntryPointFn = std::ptr::null_mut();
		
		let rc = load_assembly_and_get_function_pointer(
			entrypoint_dll_path.as_ptr(), 
			dotnet_type.as_ptr(),
			dotnet_type_method.as_ptr(),
			crate::lib::coreclr_delegates::UNMANAGEDCALLERSONLY_METHOD,
			std::ptr::null_mut::<c_void>(),
			std::mem::transmute(&mut bootstrap)
		);

        out_file.write_all(format!("load_assembly_and_get_function_pointer rc: {:x}\n", rc).as_bytes());

        out_file.write_all(if bootstrap.is_null() { "bootstrap is null".as_bytes() } else { "bootstrap isn't null!!!".as_bytes() });

		if (rc == 0 && !bootstrap.is_null())
		{
            //aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
			std::mem::transmute::<*mut ComponentEntryPointFn, ComponentEntryPointFn>(bootstrap)(std::ptr::null_mut(), 0);
		}
	}
}

fn init_net_core(config_path: OsString, mut out_file: File, settings: DoorstopBootSettings)
{
	unsafe
	{
		#[cfg(target_family = "windows")]
		if (DEBUG_WITH_MESSAGE_BOXES)
		{
			MessageBoxW(None, h!("FROM INIT NET CORE"), h!("HI"), MB_ICONEXCLAMATION);
		}
		
		//begin lib stuff scope
		{
		    let lib = load_library();
		
		    let shtuff = load_hostfxr(&lib);
		
		    let mut load_assembly_and_get_function_pointer = get_dotnet_load_assembly(config_path, std::mem::transmute(shtuff.0.try_as_raw_ptr().expect("couldn't get HostfxrGetRuntimeDelegateFn")), std::mem::transmute(shtuff.1.try_as_raw_ptr().expect("couldn't get HostfxrInitializeForRuntimeConfigFn")), std::mem::transmute(shtuff.2.try_as_raw_ptr().expect("couldn't get HostfxrCloseFn")), &out_file);

    		load_entry_point_method(load_assembly_and_get_function_pointer, &out_file, settings);
	    }
    }
} 

fn check_config_from_args() -> DoorstopBootSettings
{
	let args = std::env::args();

	let args_length = args.len();

	let mut args: Vec<String> = args.collect();

	//default values
	let mut settings = DoorstopBootSettings 
	{ 
		enabled: (true), 
		target_assembly_path: (PathBuf::from(std::env::current_exe().unwrap().parent().unwrap()).join("BepInSbox").join("core").join("BepInSbox.NET.CoreCLR.dll")), 
		target_type_namespace: ("StartupHook, BepInSbox.NET.CoreCLR".to_string()), 
		target_method_name: ("Initialize".to_string()),
		runtime_config_name: ("sbox-standalone.runtimeconfig.json".to_string()),
	};

	for mut i in 0..args_length
	{
		if (args[i].starts_with("--doorstop-"))
		{
			let doorstop_arg_name = &args[i].split_off("--doorstop-".len());

			let doorstop_arg_name = doorstop_arg_name.as_str();

			i += 1;

			let doorstop_arg_value = &args[i];

			//arg names are mostly taken from UnityDoorstop, no point changing them
			match doorstop_arg_name {
				"enabled" => (settings.enabled = doorstop_arg_value.parse::<bool>().expect("--doorstop-enabled should have value of true, or false")),
				"target-assembly-path" => (settings.target_assembly_path = PathBuf::from(doorstop_arg_value)),
				"target-type-namespace" => (settings.target_type_namespace = doorstop_arg_value.to_string()),
				"target-method-name" => (settings.target_method_name = doorstop_arg_value.to_string()),
				"runtime-config-name" => (settings.runtime_config_name = doorstop_arg_value.to_string()),
				_ => (println!("unknown arg: {doorstop_arg_name}")),
			}
		}
	}

	return settings;
}

fn init_rusty_doorstop() -> i32
{
	let mut out_file = File::create("DoorstopLog.txt").expect("Failed to create/open log file!");

	let exe_path = env::current_exe().expect("error getting current exe path? what?");

    //really cool of the PoR people (People of Rust) to not include a way to display a duration, great works thudes and thirls
	out_file.write_all((SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs().to_string() + "\n").as_bytes());
	
	out_file.write_all(format!("current module filename: {}\n", exe_path.display()).as_bytes());

	//ts sucks ass
	match (exe_path.file_name())
	{
		Some(name) => 
		{
			if (name == "sbox.exe")
			{
				out_file.write_all(b"current exe is sbox.exe, skipping loading shit, sowwie mxster facepunch\n");

				//sbox's editor is in the same folder as sbox.exe, so if we want to mod the editor and also play the game, we should just pretend like the load went fine and let the game on its merry way ^^
				return true as i32;
			}
		},

		None => return false as i32,
	}

	let settings = check_config_from_args();
	
    //let's hope they never separate the editor and the base game lol (and the standalone shit i guess?)
	let runtime_config_path = exe_path.with_file_name(&settings.runtime_config_name);

	out_file.write_all(format!("runtime config full path: {}\n", runtime_config_path.display()).as_bytes());

	init_net_core(runtime_config_path.into_os_string(), out_file, settings);

	return true as i32;
}

#[cfg(target_family = "unix")]
#[ctor(unsafe)]
fn ctor_rusty_doorstop()
{
	init_rusty_doorstop();
}

#[cfg(target_family = "windows")]
#[unsafe(no_mangle)]
pub extern "system" fn DllMain(hmodule: HMODULE, ul_reason_for_call: u32, lp_reserved: *mut c_void) -> i32
{
	unsafe 
	{
		if (ul_reason_for_call == DLL_PROCESS_ATTACH)
		{
		    let mut out_file = File::create("DoorstopLog.txt").expect("Failed to create/open log file");

			let p_string : PWSTR;
			
			match SHGetKnownFolderPath(&FOLDERID_System, KF_FLAG_DEFAULT, None)
			{
				Ok(v) => p_string = v,
				Err(_e) => return false as i32,
			}
		
			//it's probably fine if we don't check whatever
			let mut dll_path_vec: Vec<u16> = PathBuf::from(p_string.to_string().unwrap()).join(DLL_NAME.to_owned() + ".dll").to_str().unwrap().encode_utf16().collect();

            //add null terminator
            dll_path_vec.push(0);

            let dll_path = PCWSTR(dll_path_vec.as_ptr());

			//load orignal dll here later lol
            crate::xinput_def::LoadOriginalLibrary(LoadLibraryW(dll_path).expect("failed to load original xinput library, lol"));
			
			return init_rusty_doorstop() as i32;
		}
		else if (ul_reason_for_call == DLL_PROCESS_DETACH)
		{
			if (DEBUG_WITH_MESSAGE_BOXES)
			{
				MessageBoxW(None, h!("byee, byereeee"), h!("Goodbye!!!!!!"), MB_ICONINFORMATION);
			}

			return true as i32;
		}
	}

	return true as i32;
}
