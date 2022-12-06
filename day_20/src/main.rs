#[path = "../../generic/generic.rs"] mod generic;
//use std::collections::HashMap;
//use std::collections::HashSet;
//use std::ops::Add;
//use std::ops::Sub;

fn add_border(input_map: Vec<Vec<char>>, padding_char: char) -> Vec<Vec<char>> {
    let mut working_map: Vec<Vec<char>> = Vec::new();

    working_map.push(vec![padding_char; input_map[0].len()+4]);
    working_map.push(vec![padding_char; input_map[0].len()+4]);
    for row in input_map.iter() {
        let mut new_line_vec: Vec<char> = vec![padding_char; 2];
        new_line_vec.append(&mut row.clone());
        new_line_vec.append(&mut vec![padding_char; 2]);
        working_map.push(new_line_vec);
    }
    working_map.push(vec![padding_char; input_map[0].len()+4]);
    working_map.push(vec![padding_char; input_map[0].len()+4]);

    return working_map;
}

fn print_2d_map(input_map: &Vec<Vec<char>>) {
    for row in input_map {
        for x in row {
            print!("{}", x);
        }
        print!("\n");
    }
}

fn run_solution(input_file: &str, runs: usize) -> usize {
    let input_lines = generic::read_in_file(input_file);
    let algorithm: Vec<char> = input_lines[0].chars().collect();
    println!("algo len = {}", algorithm.len());

    let input_map: Vec<Vec<char>> = input_lines[2..].iter().map(|x| x.chars().collect()).collect();
    let mut working_map: Vec<Vec<char>> = input_map.clone();

    for run_index in 0..runs {
        if run_index == 0 {
            working_map = add_border(working_map, '.');
        } else if run_index % 2 == 0 {
            working_map = add_border(working_map, *algorithm.last().unwrap());
        } else {
            working_map = add_border(working_map, algorithm[0]);
        }
        
        println!("starting with");
        println!("-------------");
        print_2d_map(&working_map);
        println!("-------------");

        let mut new_map: Vec<Vec<char>> = Vec::new();

        for row_index in 0..(working_map.len() - 2) {
            let mut new_row: Vec<char> = Vec::new();
            for col_index in 0..(working_map[0].len() - 2) {
                let mut current_number: Vec<char> = Vec::new();
                for i in 0..3 {
                    current_number.append(&mut working_map[row_index + i][col_index..(col_index + 3)].to_vec());
                }

                let mut real_number: String = String::new();
                for x in current_number {
                    if x == '.' {
                        real_number.push_str("0");
                    } else if x == '#' {
                        real_number.push_str("1");
                    }
                }

                let actual_number: i32 = i32::from_str_radix(&real_number, 2).unwrap();
                //println!("actual number = {}", actual_number);
                //println!("binary_number = {}", real_number);
                //println!("actual number = {}", actual_number);
                //println!("actual number as usize = {}", actual_number as usize);
                new_row.push(algorithm[actual_number as usize]);
            }
            new_map.push(new_row);
        }

        working_map = new_map.clone();
    }
    
    println!("ending with");
    println!("-------------");
    print_2d_map(&working_map);
    println!("-------------");

    let mut count_lights_on: usize = 0;

    for row in working_map.iter() {
        count_lights_on += row.iter().filter(|x| **x == '#').collect::<Vec<&char>>().len();
    }

    println!("Lights on = {}", count_lights_on);
    return count_lights_on;
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert!(run_solution("input_example_1.txt", 2) == 35);
    }

    #[test]
    fn example_1_part_2() {
        assert!(run_solution("input_example_1.txt", 50) == 3351);
    }

    #[test]
    fn part_1() {
        assert!(run_solution("input.txt", 2) == 5663);
        //5705 is too high
        //5700 is too high
        //4000 is too low
    }

    #[test]
    fn part_2() {
        assert!(run_solution("input.txt", 50) == 19638);
        //21061 is too high
    }
}