pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub mod minihook;
pub mod process;

pub use process::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn create_process() {
        let mut p = Process::new("C:\\Windows\\System32\\notepad.exe");

        p.run();
        let p_state = p.get_state();
        assert_eq!(p_state, ProcessState::Running);

        p.terminate();
        let p_state = p.get_state();
        assert_eq!(p_state, ProcessState::Terminated);
    }
}
