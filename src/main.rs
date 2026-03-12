use minihook_rs::{minihook::MiniHook, *};
use std::io::stdin;


fn main() {
    
    let mut f = MiniHook::new();
    println!("{:?}", f);

    let p = Process::new(6432);
    println!("{:?}", p);

    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
}
