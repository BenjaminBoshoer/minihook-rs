# Minihook-rs
This project is my first attempt at combining Rust with the Windows API. I thought it would be interesting to build a small library for hooking Windows API functions.

The main goal is to hook functions from kernel32.dll using different hooking techniques, and possibly extend support to ntdll.dll later.

As a Rust beginner, I know this project won’t be fully optimized. Still, I’ll do my best to follow good practices and learn along the way.

Since this is a learning project, I won’t use LLMs to generate code. They’ll mainly assist me in answering questions and refining design ideas.

## Project Structure

The project is divided between three modules:

**Core module** - 
The core module is the orchestrator of the project. \
It is responsible for:
- Open handles to processes
- Inject processes with our DLL
- Manage hooks creation / removal

**Injector module** - This module a simple DLL injector that is being used by the core module. 
The reasons this a standalone module are **1)** to separate concerns **2)** faster compile/build time **3)** practice in creating a more complex "workflow" type project.

**Payload module** - This module will compile into a DLL that will be injected into a target process.

My end goal is to provide a simple user API for hooking functions.
```rust
let mut h = MiniHook::new();

//hook( PID,    DLL_NAME   , TARGET_FUNCTION ,    NEW_FUNCTION  )
h.hook(1234, "Kernel32.dll", "CreateProcessA", "MyCreateProcess");
```

## Project Roadmap

- **Payload module**
    - Create a simple function that can be exported
    - Once the exported function is called, extract the IAT of the process
    - Located the target function in memory
    - Perform the hook - Swap function addresses
- **Injector module**
    - Allocate memory in address space of target process
    - Copy the name of DLL into the allocated memory space
    - Call `CraeteRemoteThread` on the selected process
    - Inject the DLL into the target process
- **Core module**
    - ✅ Get handle to a running process with `OpenProcess`
    - ✅ Get Process full path with `GetModuleFileNameExA`
    - ✅ Get all loaded modules for this process with `EnumProcessModules`
    - Call the injector to inject the DLL
    - Call the exported DLL function and perform the hook
- **Expnasions**
    - Support multiple hooking methods for hooking `kernel32.dll` functions:
        - Inline hooks (manual JMP assembly)
        - IAT hooking (patch Import Address Table)
        - EAT hooking (Export Address Table for DLLs)
    - Extend hooking to `ntdll.dll` functions
    
## Useful links & Resources
Create a DLL in Rust - https://kennykerr.ca/rust-getting-started/creating-your-first-dll.html
Windows crate index for cargo.toml - https://microsoft.github.io/windows-rs/features/
DLL injections - https://relearex.wordpress.com/2017/12/26/hooking-series-part-i-import-address-table-hooking/
