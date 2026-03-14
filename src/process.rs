use std::os::windows::io::HandleOrInvalid;
use crate::hooks::*;

use windows::{
    Win32::{Foundation::{GetLastError, HANDLE, UNICODE_STRING}, System::{ProcessStatus::*, Threading::*, LibraryLoader}},
    core::*,
};
use windows::Win32::Foundation::FARPROC;
use windows::Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress};

#[derive(Debug)]
pub struct Process {
    p_path: String,
    p_name: String,
    pid: u32,
    handle: HANDLE,
    IAT_ptr: *const u8,
    hooks: Hooks,
}

impl Process {
    pub fn new(pid_t: u32) -> Result<Self> {
        let handle_t: HANDLE;
        let mut buf = vec![0u8; 260];

        unsafe{
            /// Get Process handle from PID
            match OpenProcess(PROCESS_ALL_ACCESS,true, pid_t) {
                Ok(x) => handle_t = x,
                Err(y) => return Err(y),
            }

            /// Get Process name from HANDLE
            match GetModuleFileNameExA(Some(handle_t), None, &mut buf) {
                0 => return Err(Error::new(HRESULT(-1), "Failed to get Module name")),
                _ => { },
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
        Ok( Self { p_path: path, p_name: buf, pid: pid_t, handle: handle_t, IAT_ptr: "s".to_string().as_ptr(), hooks: Hooks::new() } )

    }

    pub fn hook(&mut self) -> Result<()> {
        Self::get_dll_func_address("kernel32.dll", "CreateProcessW");

        //let s = s!("hey");

        //GetProcAddress()
        Ok(())
    }

    fn get_dll_func_address(dll: &str, f: &str) -> Result<FARPROC>{
        let dll = PCSTR(dll.to_string().as_ptr());
        let f = PCSTR(f.to_string().as_ptr());

        unsafe {
            let dll_handle = match GetModuleHandleA(dll) {
                Ok(x) => x,
                Err(y) => return Err(y),
            };

            let location = GetProcAddress(dll_handle, f);
            Ok(location)
        }
    }
}