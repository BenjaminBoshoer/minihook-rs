use crate::hooks::*;
use std::collections::HashMap;

use windows::{
    Win32::{Foundation::{GetLastError, HANDLE, UNICODE_STRING}, System::{ProcessStatus::*, Threading::*, LibraryLoader}},
    core::*,
};
use windows::Win32::Foundation::{FARPROC, HMODULE};

#[derive(Debug)]
pub struct Process {
    p_path: String,
    p_name: String,
    pid: u32,
    handle: HANDLE,
    /*loaded_modules: HashMap<String, HMODULE>,*/
    hooks: Hooks,
}

impl Process {
    pub fn new(pid_t: u32) -> Result<Self> {
        let handle_t: HANDLE;
        let mut buf = vec![0u8; 260];

        unsafe {
            /// Get Process handle from PID
            match OpenProcess(PROCESS_ALL_ACCESS, true, pid_t) {
                Ok(x) => handle_t = x,
                Err(y) => return Err(y),
            }

            /// Get Process name from HANDLE
            match GetModuleFileNameExA(Some(handle_t), None, &mut buf) {
                0 => return Err(Error::new(HRESULT(-1), "Failed to get Module name")),
                _ => {},
            }

            //let err = GetLastError();
            //println!("{:?}", err);
        }

        let buf = match String::from_utf8(buf) {
            Ok(x) => x,
            Err(y) => return Err(Error::new(HRESULT(-1), y.to_string())),
        };

        let path = buf.clone().trim_matches('\0').to_string();

        let buf = buf.trim_matches('\0').to_string().split("\\").last().unwrap().to_string();

        //todo!("tes");
        Ok(Self {
            p_path: path,
            p_name: buf,
            pid: pid_t,
            handle: handle_t,
            /*loaded_modules: HashMap::new(),*/
            hooks: Hooks::new() })
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
            let result = EnumProcessModules(self.handle,
                                            hmodule.as_mut_ptr(),
                                            (hmodule.len() * std::mem::size_of::<HMODULE>()) as u32,
                                            &mut needed_again,
            );

            match result {
                Err(y) => return Err(y),
                _ => { },
            }

            for i in 0..(needed_again / std::mem::size_of::<HMODULE>() as u32) {
                let mut name = vec!(0u8; 250);
                GetModuleFileNameExA(Some(self.handle), Some(hmodule[i as usize]), &mut name);
                let name_str = String::from_utf8(name).unwrap().trim_matches('\0').to_string();

                map.insert(String::from(name_str.clone()), hmodule[i as usize]);
            }
        }
        Ok(map)
    }
}