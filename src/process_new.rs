use std::os::windows::io::HandleOrInvalid;

use windows::{
    Win32::{Foundation::{GetLastError, HANDLE, UNICODE_STRING}, System::{ProcessStatus::*, Threading::*}},
    core::*,
};

#[derive(Debug)]
pub struct ProcessN {
    p_name: String,
    pid: u32,
    handle: HANDLE,
    IAT_ptr: *const u8,
}

impl ProcessN {
    pub fn new(pid_t: u32) -> Result<Self> {
        let handle_t: HANDLE;
        let mut buf = vec![0u8; 150];

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

        let buf = String::from_utf8(buf).unwrap();
        let buf = buf.trim_matches('\0').to_string().split("\\").last().unwrap().to_string();

        //todo!("tes");
        Ok( Self { p_name: buf, pid: pid_t, handle: handle_t, IAT_ptr: "s".to_string().as_ptr() } )

    }

}