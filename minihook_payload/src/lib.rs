use windows::{
    Win32::{
        Foundation::{HMODULE, HANDLE, UNICODE_STRING},
        System::{ProcessStatus::*, Threading::*, LibraryLoader, Diagnostics::Debug::*},
    },
    core::*,
};
use windows::Win32::Foundation::FARPROC;
use windows::Win32::System::LibraryLoader::{GetModuleFileNameA, GetModuleHandleExA};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

struct HookStatus {

}

#[unsafe(no_mangle)]
extern "system" fn Hook(module: &str, target_function: &str, payload_function: &str) -> Result<()> {
    // Get module handle
    unsafe {
        IMAGE_DATA_DIRECTORY::default();
    }

    // Get IAT


    // Find target function

    // Swap functions
    Ok(())
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
