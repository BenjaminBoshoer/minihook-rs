
#[derive(Debug)]
pub struct Hooks {
    f_original: String,
    f_original_location: String,
    f_target: String,
    f_target_location: String,
}


impl Hooks {
    pub fn new() -> Self {
        Self {  f_original: "test".to_string(),
                f_original_location: "test".to_string(), 
                f_target: "test".to_string(), 
                f_target_location: "test".to_string(),
        }
    }
}