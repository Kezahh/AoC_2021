use std::fs;

pub fn read_in_file(input_filename: &str) -> Vec<String> {
    let input_data = fs::read_to_string(input_filename).expect("unable to read file");
    let split = input_data.lines();
    let file_lines: Vec<String> = split.map(str::to_string).collect();
    return file_lines.clone();
}
