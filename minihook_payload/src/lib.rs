use windows::{
    Win32::{
        Foundation::{HMODULE, HANDLE, UNICODE_STRING},
        System::{ProcessStatus::*, Threading::*, LibraryLoader, Diagnostics::Debug::*},
    },
    core::*,
};
use windows::Win32::Foundation::FARPROC;
use windows::Win32::System::LibraryLoader::{GetModuleFileNameA, GetModuleHandleA, GetModuleHandleExA};
use windows::Win32::System::SystemServices::IMAGE_DOS_HEADER;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

struct HookStatus {

}

#[unsafe(no_mangle)]
extern "system" fn Hook(module: &str, target_function: &str, payload_function: &str) -> i32 {
    // Get handle to self
    //let p_handle = unsafe { GetCurrentProcess() };

    let mut v: Vec<HMODULE> = vec!(HMODULE::default(); 1);
    let result = unsafe{ GetModuleHandleExA(0, None, v.as_mut_ptr()).unwrap() };
    let module_handle = v[0];


    let image_dos_header = (module_handle.0 as *const IMAGE_DOS_HEADER);
    let lfanew = unsafe { (*image_dos_header).e_lfanew as usize };

    // Add 32-bit handler
    let image_nt_headers = unsafe { (image_dos_header.add(lfanew)) as *const IMAGE_NT_HEADERS64 };




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
