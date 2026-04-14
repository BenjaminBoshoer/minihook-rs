mod helper;

use crate::helper::*;
use std::ffi::CStr;
use std::io::stdin;
use std::ops::Add;
use windows::Win32::Foundation::FARPROC;
use windows::Win32::System::LibraryLoader::{
    GetModuleFileNameA, GetModuleHandleA, GetModuleHandleExA,
};
use windows::Win32::System::SystemServices::{
    IMAGE_DOS_HEADER, IMAGE_IMPORT_BY_NAME, IMAGE_IMPORT_DESCRIPTOR,
};
use windows::{
    Win32::{
        Foundation::{HANDLE, HMODULE, UNICODE_STRING},
        System::{
            Diagnostics::Debug::*, LibraryLoader, ProcessStatus::*, Threading::*,
            WindowsProgramming::*,
        },
    },
    core::*,
};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

struct HookStatus {}
#[unsafe(no_mangle)]
extern "system" fn Hook(module: &str, target_function: &str, payload_function: &str) -> i32 {
    let base = get_image_base().unwrap();

    let import_dir = get_import_dir(base);
    let function_ptr = get_function_ptr(
        base,
        import_dir,
        "kernel32.dll".to_string(),
        "CreateFileWW".to_string(),
    );

    println!("{:?}", function_ptr);

    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    return 0;
}

#[unsafe(no_mangle)]
extern "system" fn Mess(module: &str, target_function: &str, payload_function: &str) -> i32 {
    // Get handle to self
    //let p_handle = unsafe { GetCurrentProcess() };

    //let mut v: Vec<HMODULE> = vec!(HMODULE::default(); 1);
    let mut module_handle = HMODULE::default();
    let result = unsafe { GetModuleHandleExA(0, None, &mut module_handle).unwrap() };

    let base = module_handle.0 as *const u8;
    let image_dos_header = (base as *const IMAGE_DOS_HEADER);
    let lfanew = unsafe { (*image_dos_header).e_lfanew as usize };

    // 64-bit handler
    let image_nt_headers = unsafe { (base as usize + lfanew) as *const IMAGE_NT_HEADERS64 };
    // TODO: Add 32-bit handler

    let image_optional_header = unsafe { (*image_nt_headers).OptionalHeader };
    let import_rva =
        image_optional_header.DataDirectory[IMAGE_DIRECTORY_ENTRY_IMPORT.0 as usize].VirtualAddress;

    let import_dir = unsafe { (base.add(import_rva as usize)) as *const IMAGE_IMPORT_DESCRIPTOR };
    let import_dir2 = unsafe { import_dir.add(1) };

    let name = unsafe { (base as usize + (*import_dir).Name as usize) as *const PCSTR };
    //et name_2 = unsafe{name.to_string()};

    // image_nt_header = module_handle.0 + image_dos_header.e_lfanew as *const IMAGE_NT;
    // Get module handle
    // Get IAT

    println!("Success");
    // Find target function

    // Swap functions
    return 0;
}

fn get_module_handle(module: &str) -> Result<HMODULE> {
    //let result = unsafe { GetModuleHandleExA()}?;

    //unsafe { GetModuleHandleExA() }?;
    Ok(HMODULE::default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_func_address() {
        let base = get_image_base().unwrap();
        let import_dir = get_import_dir(base);
        let import_ptr = get_function_ptr(
            base,
            import_dir,
            "kernel322.dll".to_string(),
            "CreateFileW".to_string(),
        );

        assert_eq!(import_ptr, None);
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
