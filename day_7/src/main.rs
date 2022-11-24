use std::fs;

fn main() {
    let input_filename = "input2.txt";
    let input_data = fs::read_to_string(input_filename).expect("unable to read file");
    let split = input_data.lines();
    let input_string_vector: Vec<&str> = split.collect();
    //let input_string_vector = ["16,1,2,0,4,2,7,1,2,14"];

    let mut input_numbers_vector: Vec<i32> = Vec::new();
    for x in input_string_vector[0].split(",") {
        input_numbers_vector.push(x.parse().unwrap());
    }

    let mut min_num = 0;
    let mut max_num = 0;

    for num in input_numbers_vector.iter() {
        if *num > max_num {
            max_num = *num;
        } else if *num < min_num {
            min_num = *num;
        }
    }

    let mut total_fuel: Vec<i32> = vec![0; (max_num-min_num + 1) as usize];

    let mut min_fuel: i32 = 0;
    let mut min_fuel_distance: i32 = 0;
    let mut current_fuel_index: usize = 0;
    let mut target_days = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100]
    for i in min_num..(max_num + 1) {
        for num in input_numbers_vector.iter() {
            let distance: i32 = i32::abs(num - i);
            let fuel_used: i32 = (distance * (distance+1))/2;
            total_fuel[current_fuel_index] += fuel_used;
            //println!("i = {}, point = {}, distance = {}, fuel_used = {}", i, num, distance, fuel_used);
        }
        if min_fuel == 0 {
            min_fuel = total_fuel[current_fuel_index];
            min_fuel_distance = i;
        }
        if total_fuel[current_fuel_index] < min_fuel {
            min_fuel = total_fuel[current_fuel_index];
            min_fuel_distance = i;
        }
        current_fuel_index += 1;
    }

    //println!("{:?}", total_fuel);
    println!("min distance is {:?}", min_fuel_distance);
    println!("total fuel to min distance is {:?}", min_fuel);
}
