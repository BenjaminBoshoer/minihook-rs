use std::ops::Add;
use windows::{
    Win32::{
        Foundation::{HMODULE, HANDLE, UNICODE_STRING},
        System::{ProcessStatus::*, Threading::*, LibraryLoader, Diagnostics::Debug::*},
    },
    core::*,
};
use windows::Win32::Foundation::FARPROC;
use windows::Win32::System::LibraryLoader::{GetModuleFileNameA, GetModuleHandleA, GetModuleHandleExA};
use windows::Win32::System::SystemServices::{IMAGE_DOS_HEADER, IMAGE_IMPORT_DESCRIPTOR};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

struct HookStatus {

}
#[unsafe(no_mangle)]
extern "system" fn Hook(module: &str, target_function: &str, payload_function: &str) -> i32 {
    let mut base = HMODULE::default();
    let result = match unsafe { GetModuleHandleExA(0, None, &mut base).unwrap() } {
        () => { },
        _ => { return -1 ;}
    };

    let base = base.0 as *const u8;

    let image_dos_header = unsafe { base as *const IMAGE_DOS_HEADER} ;
    let image_nt_header = unsafe { (base.add((*image_dos_header).e_lfanew as usize)) as *const IMAGE_NT_HEADERS64} ;
    let image_optional_header = unsafe{ (*image_nt_header).OptionalHeader};

    let import_rva = image_optional_header.DataDirectory[IMAGE_DIRECTORY_ENTRY_IMPORT.0 as usize].VirtualAddress;
    let dll_number = image_optional_header.DataDirectory[IMAGE_DIRECTORY_ENTRY_IMPORT.0 as usize].Size;
    let dll_number = dll_number as usize / size_of::<IMAGE_IMPORT_DESCRIPTOR>();
    let import_dir = unsafe { (base as usize + import_rva as usize) as *const IMAGE_IMPORT_DESCRIPTOR};



    println!("Success");
    return 0;
}

#[unsafe(no_mangle)]
extern "system" fn Mess(module: &str, target_function: &str, payload_function: &str) -> i32 {
    // Get handle to self
    //let p_handle = unsafe { GetCurrentProcess() };

    //let mut v: Vec<HMODULE> = vec!(HMODULE::default(); 1);
    let mut module_handle = HMODULE::default();
    let result = unsafe{ GetModuleHandleExA(0, None, &mut module_handle).unwrap() };

    let base = module_handle.0 as *const u8;
    let image_dos_header = (base as *const IMAGE_DOS_HEADER);
    let lfanew = unsafe { (*image_dos_header).e_lfanew as usize };

    // 64-bit handler
    let image_nt_headers = unsafe { (base as usize + lfanew) as *const IMAGE_NT_HEADERS64 };
    // TODO: Add 32-bit handler

    let image_optional_header = unsafe { (*image_nt_headers).OptionalHeader };
    let import_rva = image_optional_header.DataDirectory[IMAGE_DIRECTORY_ENTRY_IMPORT.0 as usize].VirtualAddress;

    let import_dir = unsafe { (base.add(import_rva as usize)) as *const IMAGE_IMPORT_DESCRIPTOR};
    let import_dir2 = unsafe { import_dir.add(1)};

    let name = unsafe {(base as usize + (*import_dir).Name as usize) as *const PCSTR} ;
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
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
