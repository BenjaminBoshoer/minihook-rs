use minihook_core::{minihook::MiniHook, *};
use std::io::stdin;

use libloading::{Library, Symbol};


fn main() {

    unsafe {
        let lib = Library::new("C:\\Users\\rwxbeny\\Documents\\Rust\\minihook-rs\\target\\debug\\minihook_payload.dll").unwrap();

        let external_hook: Symbol<unsafe extern "system" fn(&str, &str, &str) -> i32> =
            lib.get(b"Hook").unwrap();

        let result = external_hook("test", "test", "test");
        println!("{result}");
    }

    /*let mut f = MiniHook::new();
    println!("{:?}", f);

    let mut p = match Process::new(5208) {
        Ok(x) => x,
        Err(y) => panic!(),
    };*/

    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
}
