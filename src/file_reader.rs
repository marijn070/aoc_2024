pub use std::fs;

pub fn get_input(input_path: &str) -> String {
    
    fs::read_to_string(input_path).expect("Unable to read file")
}
