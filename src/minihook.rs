use crate::process::*;

#[derive(Debug)]
pub struct S {
    a: i32,
    b: i32,
}

impl S {
    pub fn new() -> Self {
        Self { a: 5, b: 5 }
    }
}

pub struct MiniHook {
    a: i32,
    p_vec: Vec<Process>,
}

/*impl MiniHook {
    pub fn create_process(name: String) -> Option<PID_T>{
        Some()

    }
}*/
