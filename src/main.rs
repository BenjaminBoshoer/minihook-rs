use minihook_rs::{minihook::MiniHook, *};
use std::io::stdin;


fn main() {
    let p = ProcessN::new(1912);
    println!("{:?}", p);

    let p = ProcessN::new(4708);
    println!("{:?}", p);

    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
}
