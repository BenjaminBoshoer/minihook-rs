# Minihook-rs
This project is my first attempt at combining Rust with the Windows API. I thought it would be interesting to build a small library for hooking Windows API functions.

The main goal is to hook functions from kernel32.dll using different hooking techniques, and possibly extend support to ntdll.dll later.

As a Rust beginner, I know this project won’t be fully optimized. Still, I’ll do my best to follow good practices and learn along the way.

Since this is a learning project, I won’t use LLMs to generate code. They’ll mainly assist me in answering questions and refining design ideas.

## Project Design

I want the hooking process to be as simple as possible.

```rust
let mut h = MiniHook::new();

//hook( PID,    DLL_NAME   , TARGET_FUNCTION ,    NEW_FUNCTION  )
h.hook(1234, "Kernel32.dll", "CreateProcessA", "MyCreateProcess");
```

## Project Roadmap

- **Implement a `Process` struct**
    - ✅ Get handle to a running process with `OpenProcess`
    - ✅ Get Process full path with `GetModuleFileNameExA`
    - Get the IAT Address / Struct
    - Perform the hook - Swap function addresses
- **Implement a `MiniHook` struct**
    - ✅ Define a Hashmap<u32, Process>
    - Implement `hook()`
        - Check if target process already exists in map
        - If target process exists, check that the process didn't change
        - Call `Process::hook()`
- **Testing**
    - Test Process creation
    - Test creating Process -> killing it -> trying to hook.
- **Expnasions**
    - Support multiple hooking methods for hooking `kernel32.dll` functions:
        - Inline hooks (manual JMP assembly)
        - IAT hooking (patch Import Address Table)
        - EAT hooking (Export Address Table for DLLs)
    - Extend hooking to `ntdll.dll` functions
    

