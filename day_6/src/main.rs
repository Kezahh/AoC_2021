use std::fs;

fn main() {
    let input_filename = "input.txt";
    let input_data = fs::read_to_string(input_filename).expect("unable to read file");
    let split = input_data.lines();
    let input_string_vector: Vec<&str> = split.collect();
    //let input_string_vector = ["3,4,3,1,2"];
    
    let mut input_fish_vector: Vec<usize> = Vec::new();
    for x in input_string_vector[0].split(",") {
        input_fish_vector.push(x.parse().unwrap());
    }
    //println!("{:?}", input_fish_vector);

    let total_days = 256;
    let divisor = 4;

    // let mut current_fish_vector: Vec<usize> = Vec::new();
    // for fish in initial_fish_vector.iter() {
    //     current_fish_vector.push(*fish);
    // }

    // for day in 0..total_days {
    //     // increase fish
    //     //println!("{:?}", current_fish_vector);
    //     let mut new_fish_count = 0;
    //     for j in 0..current_fish_vector.len() {
    //         if current_fish_vector[j] == 0 {
    //             current_fish_vector[j] = 6;
    //             new_fish_count += 1;
    //         } else {
    //             current_fish_vector[j] -= 1;
    //         }
    //     }
    //     for _j in 0..new_fish_count {
    //         current_fish_vector.push(8);
    //     }
    //     println!("day {}", day);
    //     //println!("day {}: {:?}", day, current_fish_vector);
    //     //break;
    // }

    let mut fish_results: Vec<Vec<usize>> = vec![Vec::new(); 9];

    for i in 0..fish_results.len() {
        println!("Running fish {}", i);
        fish_results[i] = get_fish_vector(i, total_days/divisor);
    }

    println!("{:?}", fish_results);

    let mut initial_fish: Vec<Vec<usize>> = vec![vec![0; 9]; 9];
    for i in 0..initial_fish.len() {
        initial_fish[i][i] = 1;
    }

    println!("{:?}", initial_fish);

    for _i in 0..divisor {
        for j in 0..9 {
            // [1,0,0,0,0,0,0,0,0]
            let initial_fish_vector = &initial_fish[j];
            let mut new_fish_vector: Vec<usize> = vec![0; 9];
            for target_fish_index in 0..9 {
                let fish_count = initial_fish_vector[target_fish_index];
                for fish_result_index in 0..9 {
                    new_fish_vector[fish_result_index] += fish_count * fish_results[target_fish_index][fish_result_index];
                }
            }
            initial_fish[j] = new_fish_vector;
        }
    }
    println!("{:?}", initial_fish);

    let mut total_count = 0;
    for fish in input_fish_vector.iter() {
        for fish_count in initial_fish[*fish].iter() {
            total_count += fish_count;
        }
    }
    println!("Total fish = {:?}", total_count);
    //println!("{}",current_fish_vector.len());
}


fn get_fish_vector(start_number: usize, total_days: usize) -> Vec<usize> {
    let mut current_fish_vector: Vec<usize> = Vec::new();

    current_fish_vector.push(start_number);

    for day in 0..total_days{
        // increase fish
        //println!("{:?}", current_fish_vector);
        let mut new_fish_count = 0;
        for j in 0..current_fish_vector.len() {
            if current_fish_vector[j] == 0 {
                current_fish_vector[j] = 6;
                new_fish_count += 1;
            } else {
                current_fish_vector[j] -= 1;
            }
        }
        for _j in 0..new_fish_count {
            current_fish_vector.push(8);
        }
        //println!("day {}", day);
        //println!("day {}: {:?}", day, current_fish_vector);
        //break;
    }

    let mut fish_set_vector = vec![0; 8+1];

    for fish in current_fish_vector.iter() {
        fish_set_vector[*fish] += 1;
    }

    return fish_set_vector;
}