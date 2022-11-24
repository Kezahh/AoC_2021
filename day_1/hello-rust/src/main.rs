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

    println!("{}", input_vector[1]);
    println!("{}", input_vector[1] > input_vector[0]);


    //let input_vector = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    
    let mut increase_sum = 0;

    for i in 0..input_vector.len() {
        if i != 0 {
            if input_vector[i] > input_vector[i-1] {
                //println!("{}: {}", i, input_vector[i]);
                //print!("{}", input_vector[i]);
                //println!(" has increased");
                increase_sum += 1;
            }
        }
    }

    println!("{} have increased", increase_sum);
}
