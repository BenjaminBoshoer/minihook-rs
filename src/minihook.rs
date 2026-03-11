use crate::process::*;
use std::collections::HashMap;
use crate::process_new::*;

#[derive(Debug)]
pub struct MiniHook {
    process: HashMap<u32, Process>,
    //t_iat: IAT,
}

/*impl MiniHook {
    pub fn new(p_name: &str) -> Self {
        let p = Process::new(p_name);
        Self { process: p }
    }
}*/

#[derive(Debug)]
pub struct IAT {}
