use std::fs;

fn main() {
    let input_filename = "input.txt";
    let input_data = fs::read_to_string(input_filename).expect("unable to read file");
    let split = input_data.lines();
    let input_string_vector: Vec<&str> = split.collect();
    let mut input_vector: Vec<i32> = Vec::with_capacity(input_string_vector.len());

    for i in 0..input_string_vector.len() {
        //println!("{}", input_string_vector[i]);
        input_vector.push(input_string_vector[i].parse().unwrap());
    }

    let mut sliding_sums: Vec<i32> = Vec::with_capacity(input_vector.len()-2);

    for i in 0..(input_vector.len()-2) {
        sliding_sums.push(input_vector[i] + input_vector[i+1] + input_vector[i+2]);
    }
    

    let mut increase_count = 0;
    for i in 1..sliding_sums.len() {
        if sliding_sums[i] > sliding_sums[i-1] {
            increase_count += 1;
        }
    }

    println!("{} increases", increase_count);
}
