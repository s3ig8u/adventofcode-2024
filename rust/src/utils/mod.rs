use std::fs;

pub fn read_input_file(path: &str) -> String {
    println!("Reading input file from {}", path);
    fs::read_to_string(path).unwrap()
}
