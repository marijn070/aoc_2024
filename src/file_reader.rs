pub use std::fs;

pub fn get_input(input_path: &str) -> String {
    let data = fs::read_to_string(input_path).expect("Unable to read file");
    data
}
