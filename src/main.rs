use minihook_rs::{minihook::MiniHook, *};
use std::io::stdin;


fn main() {
    
    let mut f = MiniHook::new();
    println!("{:?}", f);

    let mut p = match Process::new(9616) {
        Ok(x) => x,
        Err(y) => panic!(),
    };
    let result = match p.get_modules() {
        Ok(x) => x,
        Err(y) => panic!(),
    };

    for key in result.keys() {
        println!("{}", key);
    }

    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
}
