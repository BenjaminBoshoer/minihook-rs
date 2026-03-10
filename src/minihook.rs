use crate::process::*;

#[derive(Debug)]
pub struct MiniHook {
    process: Process,
    //t_iat: IAT,
}

impl MiniHook {
    pub fn new(p_name: &str) -> Self {
        let p = Process::new(p_name);
        Self { process: p }
    }
}

#[derive(Debug)]
pub struct IAT {}
