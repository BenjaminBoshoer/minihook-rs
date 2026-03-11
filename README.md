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
    - ✅ Create a process by calling `CreateProcessA`
    - ✅ Get handle to a running process with `OpenProcess`
- **Implement basic DLL injection**
    - use `VirtualAllocEx` to allocate memory in target process
    - write shellcode/path to our hooked DLL using WriteProcessMemory
    - Create remote thread with `CreateRemoteThread` to call `LoadLibraryA`
- **Hook Implementation**
    - Support multiple hooking methods for hooking `kernel32.dll` functions:
        - Inline hooks (manual JMP assembly)
        - IAT hooking (patch Import Address Table)
        - EAT hooking (Export Address Table for DLLs)
    - Error handling & cleanup
        - Unhook in DLL unload
- **Expnasions**
    - Extend hooking to `ntdll.dll` functions
    - Present the IAT table
