use std::ffi::CStr;
use windows::Win32::Foundation::HMODULE;
use windows::Win32::System::Diagnostics::Debug::{IMAGE_DIRECTORY_ENTRY_IMPORT, IMAGE_NT_HEADERS64};
use windows::Win32::System::LibraryLoader::GetModuleHandleExA;
use windows::Win32::System::SystemServices::{IMAGE_DOS_HEADER, IMAGE_IMPORT_DESCRIPTOR};

pub fn ptr_to_str(ptr: *const i8) -> Option<String> {
    let str = unsafe { CStr::from_ptr(ptr).to_string_lossy().into_owned() };
    Some(str)
}

pub fn get_import_dir(base: *const u8) -> *mut IMAGE_IMPORT_DESCRIPTOR{
    let image_dos_header = unsafe { base as *const IMAGE_DOS_HEADER} ;
    let image_nt_header = unsafe { (base.add((*image_dos_header).e_lfanew as usize)) as *const IMAGE_NT_HEADERS64} ;
    let image_optional_header = unsafe{ (*image_nt_header).OptionalHeader};

    let import_rva = image_optional_header.DataDirectory[IMAGE_DIRECTORY_ENTRY_IMPORT.0 as usize].VirtualAddress;
    let dll_number = image_optional_header.DataDirectory[IMAGE_DIRECTORY_ENTRY_IMPORT.0 as usize].Size;
    let mut import_dir = unsafe { (base as usize + import_rva as usize) as *mut IMAGE_IMPORT_DESCRIPTOR};
    import_dir
}

pub fn get_image_base() -> Option<*const u8> {
    let mut base = HMODULE::default();
    let result = match unsafe { GetModuleHandleExA(0, None, &mut base).unwrap() } {
        () => { },
        _ => { return None }
    };

    let base = base.0 as *const u8;
    Some(base)
}