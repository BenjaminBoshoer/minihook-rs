use minihook_rs::{minihook::MiniHook, *};
use std::io::stdin;


fn main() {
    
    let mut f = MiniHook::new();
    println!("{:?}", f);

    let mut p = match Process::new(10916) {
        Ok(x) => x,
        Err(y) => panic!(),
    };

    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
}
