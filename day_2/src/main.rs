use std::fs;

fn main() {
    let input_filename = "input.txt";
    let input_data = fs::read_to_string(input_filename).expect("unable to read file");
    let split = input_data.lines();
    let input_string_vector: Vec<&str> = split.collect();
    

    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim: i32 = 0;

    for x in input_string_vector {
        let line_split = x.split(" ");
        let line_split_result: Vec<&str> = line_split.collect();
        let direction = line_split_result[0];
        let distance: i32 = line_split_result[1].parse().unwrap();

        if direction == "forward" {
            horizontal_position += distance;
            depth += aim * distance;
        } else if direction == "backwards" {
            horizontal_position -= distance;
            depth += aim * distance;
        } else if direction == "up" {
            aim -= distance;
            // depth -= distance;
        } else if direction == "down" {
            aim += distance;
            // depth += distance;
        }
    }
  
    println!("horizontal_position = {}, depth = {}", horizontal_position, depth);
    println!("product = {}", horizontal_position * depth);
}
