use std::fs;
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Octopus{
    value: usize,
    adjacent_octopus: Vec<usize>,
    flashed: bool,
}

fn main() {
    let input_filename = "input.txt";
    let input_data = fs::read_to_string(input_filename).expect("unable to read file");
    let split = input_data.lines();
    let input_lines: Vec<&str> = split.collect();
    let mut input_map: Vec<Vec<usize>> = Vec::new();

    for row in input_lines.iter() {
        let mut row_vector: Vec<usize> = Vec::new();
        for c in row.chars() {
            row_vector.push((c as usize)-48); //48 is 0 in ASCII
        }
        input_map.push(row_vector);
    }

    let mut octopus_list: Vec<Octopus> = Vec::new();
    for row in input_map.iter() {
        for num in row.iter() {
            octopus_list.push(Octopus{
                value: *num,
                flashed: false,
                adjacent_octopus: Vec::new(),
            });
        }
    }

    for row_index in 0..input_map.len() {
        for col_index in 0..input_map[row_index].len() {
            //println!("{:?}", octopus);
            let mut start_row_index: usize;
            let mut end_row_index: usize;
            let mut start_col_index: usize;
            let mut end_col_index: usize;

            if row_index == 0 {
                start_row_index = 0;
            } else {
                start_row_index = row_index - 1;
            }

            if col_index == 0 {
                start_col_index = 0;
            } else {
                start_col_index = col_index - 1;
            }

            if row_index == input_map.len() -1 {
                end_row_index = row_index;
            } else {
                end_row_index = row_index + 1;
            }
            if col_index == input_map[row_index].len() - 1 {
                end_col_index = col_index;
            } else {
                end_col_index = col_index + 1;
            }
            
            if row_index == 0 {
                println!("start_row_index = {}, end_row_index = {}, start_col_index = {}, end_col_index ={}", start_row_index, end_row_index, start_col_index, end_col_index);
            }
            let mut adj_octopuses: Vec<usize> = Vec::new();
            for adj_row in start_row_index..(end_row_index + 1) {
                for adj_col in start_col_index..(end_col_index+1) {
                    if !(adj_row == row_index && adj_col == col_index) {
                        //octopus_map[row_index][col_index].adjacent_octopus.push(adj_octopus);
                        adj_octopuses.push((adj_row * input_map[row_index].len()) + adj_col);
                    }
                }
                octopus_list[(row_index * input_map.len()) + col_index].adjacent_octopus = adj_octopuses.clone();
            }
        }
    }

    for row_index in 0..input_map.len() {
        for col_index in 0..input_map[0].len() {
            print!("{}", octopus_list[(row_index * 10) + col_index].value);
        }
        print!("\n");
    }
    print!("\n");

    //FLASH TIME
    let days: usize = 495;
    let mut target_days: Vec<usize> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    let mut flash_count = 0;
    let mut all_flash_step: Vec<usize> = Vec::new();
    for _day in 0..days {
        for octopus in octopus_list.iter_mut() {
            octopus.value += 1;
        }

        let mut octopus_count = 0;
        while octopus_count != octopus_list.len() {
            octopus_count = 0;
            for list_index in 0..octopus_list.len() {
                if octopus_list[list_index].value > 9 && !octopus_list[list_index].flashed {
                    octopus_list[list_index].flashed = true;
                    for octopus_index in octopus_list[list_index].adjacent_octopus.clone() {
                        octopus_list[octopus_index].value += 1;
                    }
                } else {
                    octopus_count += 1;
                }
            }
        }

        let mut all_flash = true;
        for octopus in octopus_list.iter_mut() {
            if octopus.value > 9 {
                octopus.value = 0;
                octopus.flashed = false;
                flash_count += 1;
            } else {
                all_flash = false;
            }
            assert_eq!(octopus.flashed, false)
        }
        if all_flash {
            all_flash_step.push(_day + 1);
        }

        if target_days.contains(&(_day + 1)) {
            println!("Day {}:", _day+1);
            for row_index in 0..input_map.len() {
                for col_index in 0..input_map[0].len() {
                    print!("{}", octopus_list[(row_index * 10) + col_index].value);
                }
                print!("\n");
            }
            print!("\n");
        }
    }

    println!("Total flashes: {}", flash_count);
    println!("All flashed on days {:?}", all_flash_step);
}
