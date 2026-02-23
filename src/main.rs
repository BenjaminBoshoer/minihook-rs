use minihook_rs::*;

fn main() {
    println!("Let's do an API call!");

    let mut p1 = Process::new("C:\\Windows\\System32\\notepad.exe");
    match p1.run() {
        Ok(()) => { println!("Process created successfuly! PID: {}, Handle: {:?}", p1.get_pid(), p1.get_handle()) }
        Err(x) => { println!("Error! Creating process failed with the following error:{}", x) }
    }

    println!("{:?}", p1);
}
