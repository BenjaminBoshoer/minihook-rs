use crate::process::*;
use std::collections::HashMap;
use crate::process::*;

#[derive(Debug)]
pub struct MiniHook {
    process: HashMap<u32, Process>,
}

impl MiniHook {
    pub fn new() -> Self {
        let map: HashMap<u32, Process> = HashMap::new();
        Self { process: map }
    }

    pub fn hook(&self, pid:u32, dll: &str, f_origin: &str, f_target: &str) -> std::io::Result<()> {
        if self.process.contains_key(&pid) {
            todo!();
        }
        
        Ok(())
    }
}

#[derive(Debug)]
pub struct IAT {}
