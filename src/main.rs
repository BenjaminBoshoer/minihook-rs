use minihook_rs::*;

fn main() {
    println!("Hello, world!\nLet's do an API call!");
    //unsafe {println!("{:?}", pcstr_name.0)};

    let mut p1 = Process::new("C:\\Windows\\System32\\notepad.exe");
    /*match p1.run() {
        Ok(x) => { println!("Everything was good") }
        Err(x) => { println!("Error! {}", x,) }
    }*/

    p1.run();

    println!("{:?}", p1);
}
