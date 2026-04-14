use minihook_core::{minihook::MiniHook, *};
use std::io::stdin;

use libloading::{Library, Symbol};
use windows::core::s;
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxExA, MB_OK};

fn main() {
    unsafe {
        let lib = Library::new(
            "C:\\Users\\rwxbeny\\Documents\\Rust\\minihook-rs\\target\\debug\\minihook_payload.dll",
        )
        .unwrap();

        let external_hook: Symbol<unsafe extern "system" fn(&str, &str, &str) -> i32> =
            lib.get(b"Hook").unwrap();

        let result = external_hook("user32.dll", "MessageBoxExA", "test");
        println!("{result}");
    }

    let hooked_caption = s!("Hello, ");
    let hooked_text = s!("World!");
    let result = unsafe { MessageBoxExA(None, hooked_text, hooked_caption, MB_OK, 0) };

    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
}
