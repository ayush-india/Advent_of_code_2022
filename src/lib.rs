use std::fs;
pub fn get_content(path: &str) -> String {
    fs::read_to_string(path).expect("Unable to read file")
}
