use minihook_core::{minihook::MiniHook, *};
use std::io::stdin;


fn main() {
    
    let mut f = MiniHook::new();
    println!("{:?}", f);

    let mut p = match Process::new(12156) {
        Ok(x) => x,
        Err(y) => panic!(),
    };

    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
}
