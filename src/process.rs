use std::str::FromStr;

use windows::{Win32::System::Threading::*, core::*};

use crate::process;

#[derive(Debug)]
pub enum ProcessState {
    Running,
    Idle,
    Terminated,
    Pending,
    Empty,
}

#[derive(Debug)]
pub struct Process {
    p_name: String,
    pid: i32,
    state: ProcessState,
}

impl Process {
    // Fill the strut with default values except the process name.
    pub fn new(name: &str) -> Self {
        let p1 = Process::default();

        Self {
            p_name: name.to_owned(),
            ..p1
        }
    }

    // Calling the run() will actually create the process and run it.
    pub fn run(&mut self) -> Result<()> {
        match self.state {
            ProcessState::Empty => {}
            _ => {
                return Err(Error::new(HRESULT(-1), "test"));
            }
        }

        let p_name_ptr: *const u8 = self.p_name.as_ptr();
        let p_name_ptr = PCSTR(p_name_ptr);

        unsafe {
            let si = STARTUPINFOA::default();
            let mut pi = PROCESS_INFORMATION::default();

            let status = CreateProcessA(
                p_name_ptr,
                None,
                None,
                None,
                false,
                PROCESS_CREATION_FLAGS(0),
                None,
                None,
                &si,
                &mut pi,
            );

            match status {
                Ok(_) => self.state = ProcessState::Running,
                Err(x) => return Err(x),
            }
        }

        return Ok(());
    }
}

// Default implementation for the Process struct
impl Default for Process {
    fn default() -> Self {
        Self {
            p_name: "notepad.exe".to_owned(),
            pid: -1,
            state: ProcessState::Empty,
        }
    }
}
