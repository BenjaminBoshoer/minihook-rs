mod helper;
mod hooks;

use crate::helper::*;
use crate::hooks::*;
use std::io::stdin;
use std::ops::Add;
use std::ptr;
use windows::{
    Win32::{
        Foundation::{HANDLE, HMODULE, UNICODE_STRING},
        System::{Diagnostics::Debug::*, Threading::*, WindowsProgramming::*},
    },
    core::*,
};
use windows::Win32::System::Memory::{VirtualProtect, PAGE_READWRITE, PAGE_EXECUTE_READ, PAGE_PROTECTION_FLAGS};


#[unsafe(no_mangle)]
extern "system" fn Hook(module: &str, target_function: &str, hook_func_ptr: *const i64) -> i32 {
    let base = get_image_base().unwrap();

    let import_dir = get_import_dir(base);
    /*let function_ptr = get_function_ptr(
        base,
        import_dir,
        "kernel32.dll".to_string(),
        "CreateFileWW".to_string(),
    );*/

    let function_ptr = get_function_ptr(
        base,
        import_dir,
        module.to_string(),
        target_function.to_string(),
    ).unwrap();

    let page_start = (function_ptr as usize) & !0xFFF; // round down to 4K boundary
    let mut old_prot: Vec<PAGE_PROTECTION_FLAGS> = vec!(PAGE_PROTECTION_FLAGS::default(); 1);
    unsafe {
        // Make the page writable
        VirtualProtect(
            page_start as *mut std::ffi::c_void,
            4096,
            PAGE_READWRITE,
            old_prot.as_mut_ptr()).unwrap()

    }

    let hook_fn: fn() -> i8 = MyMessageBoxExA;
    let hook_fn_addr = hook_fn as usize;

    unsafe {ptr::write(function_ptr as *mut usize, hook_fn_addr)};

    println!("new function ptr: {:?}", function_ptr);

    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// This test tries to get the import address of a function from a non-existent DLL name
    fn non_existent_dll() {
        let base = get_image_base().unwrap();
        let import_dir = get_import_dir(base);
        let import_ptr = get_function_ptr(
            base,
            import_dir,
            "kernel52.dll".to_string(),
            "CreateFileW".to_string(),
        );

        assert_eq!(import_ptr, None);
    }

    #[test]
    /// This test tries to get the import address of a non-existent function
    fn non_existent_function() {
        let base = get_image_base().unwrap();
        let import_dir = get_import_dir(base);
        let import_ptr = get_function_ptr(
            base,
            import_dir,
            "kernel32.dll".to_string(),
            "WrongFunctionName".to_string(),
        );

        assert_eq!(import_ptr, None);
    }
}
