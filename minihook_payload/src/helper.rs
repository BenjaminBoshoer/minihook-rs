use std::ffi::CStr;
use windows::Win32::Foundation::HMODULE;
use windows::Win32::System::Diagnostics::Debug::{IMAGE_DIRECTORY_ENTRY_IMPORT, IMAGE_NT_HEADERS64};
use windows::Win32::System::LibraryLoader::GetModuleHandleExA;
use windows::Win32::System::SystemServices::{IMAGE_DOS_HEADER, IMAGE_IMPORT_BY_NAME, IMAGE_IMPORT_DESCRIPTOR};
use windows::Win32::System::WindowsProgramming::IMAGE_THUNK_DATA64;

pub fn ptr_to_str(ptr: *const i8) -> Option<String> {
    let str = unsafe { CStr::from_ptr(ptr).to_string_lossy().into_owned() };
    Some(str)
}

pub fn get_target_thunk(base: *const u8, import_dir: *const IMAGE_IMPORT_DESCRIPTOR, target_name: String) -> Option<*const IMAGE_THUNK_DATA64> {
    let mut dll_ptr = import_dir;
    //let dll_name = ptr_to_str(dll_name).unwrap();

    unsafe {
        loop {
            if (*dll_ptr).FirstThunk == 0 && (*dll_ptr).Anonymous.OriginalFirstThunk == 0  {
                return None;
            }

            let dll_name_ptr = base as usize + (*dll_ptr).Name as usize;
            let dll_name = ptr_to_str(dll_name_ptr as *const i8).unwrap();

            if dll_name == target_name {
                let mut ilt_thunk_ptr = (base as usize + (*dll_ptr).Anonymous.OriginalFirstThunk as usize) as *mut IMAGE_THUNK_DATA64;
                loop {
                    let mut ilt_thunk_name_ptr = (base as usize + (*ilt_thunk_ptr).u1.AddressOfData as usize) as *const IMAGE_IMPORT_BY_NAME;
                    let ilt_thunk_name = ptr_to_str((*ilt_thunk_name_ptr).Name.as_ptr());
                    ilt_thunk_ptr = ilt_thunk_ptr.add(1);
                    println!("");
                }
            }
            dll_ptr = dll_ptr.add(1);
        }
    }

    //Some(unsafe{ (base as usize + (*import_dir).Anonymous.OriginalFirstThunk as usize) as *const IMAGE_THUNK_DATA64 })

    //while let name
    None
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

pub fn get_address_thunk_by_function_name(base_ptr: *const i8, dll_name: String, funtion_name: String) -> *mut IMAGE_THUNK_DATA64 {
    let mut address_thunk = IMAGE_THUNK_DATA64::default();

    return &mut address_thunk;
}