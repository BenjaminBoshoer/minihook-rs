use crate::hooks::*;
use std::collections::HashMap;
use windows::Win32::Foundation::FARPROC;
use windows::Win32::System::LibraryLoader::LoadLibraryA;
use windows::{
    Win32::{
        Foundation::{HANDLE, HMODULE, UNICODE_STRING},
        System::{
            Diagnostics::Debug::*, LibraryLoader, ProcessStatus::*, SystemServices::*, Threading::*,
        },
    },
    core::*,
};

#[derive(Debug)]
pub struct Process {
    p_path: String,
    p_name: String,
    pid: u32,
    handle: HANDLE,
    /*loaded_modules: HashMap<String, HMODULE>,*/
    base_addr: HMODULE,
    hooks: Hooks,
}

/// Public High-Level API
impl Process {
    pub fn new(pid_t: u32) -> Result<Self> {
        //Get Process handle
        let handle_t = unsafe { OpenProcess(PROCESS_ALL_ACCESS, true, pid_t)? };

        // Get Process path from handle
        let path = Process::get_process_path(handle_t)?;

        // Extract Process name from the path
        let name = path.split("\\").last().unwrap().to_string();

        // Get Process base address
        let base_address = Process::get_base_address(handle_t)?;

        let mut dos_header = IMAGE_DOS_HEADER::default();
        let test = base_address.0 as *mut IMAGE_DOS_HEADER;

        let iat = Process::get_IAT(base_address);

        Ok(Self {
            p_path: path,
            p_name: name,
            pid: pid_t,
            handle: handle_t,
            /*loaded_modules: HashMap::new(),*/
            base_addr: base_address,
            hooks: Hooks::new(),
        })
    }

    pub fn hook(&mut self) -> Result<()> {
        //let result = self.get_dll_func_address("kernel32.dll", "CreateFileW");
        //println!("{:?}", result);
        //let s = s!("hey");

        //GetProcAddress()
        Ok(())
    }

    pub fn get_modules(&mut self) -> Result<(HashMap<String, HMODULE>)> {
        let mut hmodule = vec![HMODULE::default(); 1024];
        let mut needed_again: u32 = 0;

        let mut map: HashMap<String, HMODULE> = HashMap::new();

        unsafe {
            let result = EnumProcessModules(
                self.handle,
                hmodule.as_mut_ptr(),
                (hmodule.len() * std::mem::size_of::<HMODULE>()) as u32,
                &mut needed_again,
            );

            match result {
                Err(y) => return Err(y),
                _ => {}
            }

            for i in 0..(needed_again / std::mem::size_of::<HMODULE>() as u32) {
                let mut name = vec![0u8; 250];
                GetModuleFileNameExA(Some(self.handle), Some(hmodule[i as usize]), &mut name);
                let name_str = String::from_utf8(name)
                    .unwrap()
                    .trim_matches('\0')
                    .to_string();

                map.insert(String::from(name_str.clone()), hmodule[i as usize]);
            }
        }
        Ok(map)
    }

    pub fn load_lib() -> () {
        let result = unsafe {
            LoadLibraryA(s!(
                "C:\\Users\\rwxbeny\\Documents\\Rust\\minihook-rs\\target\\debug\\minihook_payload.dll"
            ))
        };
    }
}

/// Private helper functions
impl Process {
    /// Get full Process path from a process handle
    fn get_process_path(p_handle: HANDLE) -> Result<String> {
        unsafe {
            let mut buf: Vec<u8> = vec![0u8; 256];

            /// Get Process name from HANDLE
            match GetModuleFileNameExA(Some(p_handle), None, &mut buf) {
                0 => return Err(Error::new(HRESULT(-1), "Failed to get Module name")),
                _ => {}
            }

            let buf = String::from_utf8(buf)
                .unwrap()
                .trim_matches('\0')
                .to_string();
            Ok(buf)
        }
    }

    /// Get the base address of a process
    fn get_base_address(p_handle: HANDLE) -> Result<HMODULE> {
        let mut v: Vec<HMODULE> = vec![HMODULE::default(); 1024];
        //let v_size = size_of::<std::mem::size_of<>()>();
        let v_size = std::mem::size_of::<HMODULE>() * v.len();
        let v_size = v_size as u32;
        let mut modules_size: u32 = 0;

        unsafe {
            let result =
                match EnumProcessModules(p_handle, v.as_mut_ptr(), v_size, &mut modules_size) {
                    Err(y) => return Err(y),
                    _ => {}
                };

            let base_addr = v[0];
            return Ok(base_addr);
        }
    }

    fn get_IAT(base_addr: HMODULE) -> Result<u64> {
        let mut size: Vec<u32> = vec![0u32, 4];
        let mut section_header = IMAGE_SECTION_HEADER::default();
        let imports_dir_base_addr = unsafe {
            ImageDirectoryEntryToDataEx(
                base_addr.0,
                false,
                IMAGE_DIRECTORY_ENTRY_EXPORT,
                size.as_mut_ptr(),
                None,
            )
        };
        Ok(0 as u64)
    }
}
