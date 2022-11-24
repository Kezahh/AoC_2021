use std::fs;

fn main() {
    let input_filename = "input.txt";
    let input_data = fs::read_to_string(input_filename).expect("unable to read file");
    let split = input_data.lines();
    let all_lines: Vec<&str> = split.collect();
    let mut input_map: Vec<Vec<i32>> = Vec::new();

    input_map.push(vec![9; all_lines[0].len()+2]);
    for line in all_lines.iter() {
        let mut row_nums: Vec<i32> = Vec::new();
        row_nums.push(9);
        for target_char in line.chars() {
            row_nums.push((target_char as i32)-48); //48 is ASCII for 0.
        }
        row_nums.push(9);
        input_map.push(row_nums);
    }
    input_map.push(vec![9; all_lines[0].len()+2]);
    
    // for row in input_map.iter() {
    //     println!("{:?}", row);
    // }


    let mut low_points: Vec<i32> = Vec::new();
    for row_index in 1..(input_map.len()-1) {
        for col_index in 1..(input_map[0].len()-1) {
            let left_num: i32 = input_map[row_index][col_index-1];
            let target_num: i32 = input_map[row_index][col_index];
            let right_num: i32 = input_map[row_index][col_index+1];

            // if row_index == 1 {
            //     println!("{} {} {}", left_num, target_num, right_num);
            // }
            
            if (target_num < left_num) && (target_num < right_num) {
                let top_num: i32 = input_map[row_index-1][col_index];
                let bot_num: i32 = input_map[row_index+1][col_index];
                // if row_index == 1 {
                //     println!("b{} {} t{}", bot_num, target_num, top_num);
                // }
                if (target_num < top_num) && (target_num < bot_num) {
                    low_points.push(target_num);
                }

                
            }
        }
    }

    let mut risk_level: i32 = 0;
    for x in low_points.iter() {
        risk_level += *x + 1;
    }

    println!("low points = {:?}", low_points);
    println!("Total risk level = {}", risk_level);

    
    let mut new_map: Vec<Vec<char>> = vec![vec!['a'; input_map[0].len()-2]; input_map.len()-2];
    let mut new_map: Vec<Vec<usize>> = vec![vec![10; input_map[0].len()-2]; input_map.len()-2];

    let mut row_index = 1;
    //let mut current_char = 'b';
    //let mut max_char = 'b';
    let mut current_char = 10;
    let mut max_char = 10;
    while row_index < (input_map.len()-1) {
        let mut col_index = 1;
        while col_index < (input_map[0].len() - 1) {
            let col_start_index: usize;
            let col_end_index: usize;
            if input_map[row_index][col_index] != 9 {
                col_start_index = col_index-1;

                println!("current char start: {}", current_char);
                //current_char = char::from_u32((current_char as u32) + 1).unwrap();
                current_char += 1;
                println!("current char end: {}", current_char);
                //max_char = char::from_u32((max_char as u32) + 1).unwrap();
                max_char += 1;

                while input_map[row_index][col_index] != 9 {
                    new_map[row_index-1][col_index-1] = current_char;
                    col_index += 1;
                }
                col_end_index = col_index-1;

                let mut found_above = false;
                //let mut chars_above: Vec<char> = Vec::new();
                let mut chars_above: Vec<usize> = Vec::new();

                
                if row_index > 1{
                    for above_col_index in col_start_index..(col_end_index) {
                        //println!("row is {}, col is {}", row_index, above_col_index);
                        if new_map[row_index - 2][above_col_index] != 9 {
                            if !chars_above.contains(&new_map[row_index-2][above_col_index]){
                                chars_above.push(new_map[row_index-2][above_col_index]);
                                found_above = true;
                            }
                        }
                    }

                    if found_above {
                        println!("above chars are {:?}", chars_above);
                        for above_char in chars_above.iter() {
                            let mut above_row_index = row_index-2; // minus 2, index of row_index is already 1 away from new_map
                            //println!("looking for {:?}", above_char);
                            //println!("above row is {:?}", new_map[above_row_index]);
                            while new_map[above_row_index].contains(above_char) {
                                for x in 0..new_map[above_row_index].len() {
                                    if new_map[above_row_index][x] == *above_char {
                                        new_map[above_row_index][x] = current_char;
                                    }
                                }
                                if above_row_index == 0 {
                                    break;
                                }
                                above_row_index -= 1;
                            }
                        }

                        let mut col_index2 = 0;
                        while col_index2 < col_index {
                            let mut found_current_above = false;
                            let col_start_index2: usize;
                            let col_end_index2: usize;

                            if new_map[row_index -1][col_index2] != 9 {
                                col_start_index2 = col_index2;
                                while new_map[row_index -1][col_index2] != 9 {
                                    //println!("col_index is {}, col_index2 is {}", col_index, col_index2);
                                    if new_map[row_index-2][col_index2] == current_char {
                                        found_current_above = true;
                                    }
                                    col_index2 += 1;
                                    if col_index2 == col_index -1 {
                                        break;
                                    }
                                }
                                col_end_index2 = col_index2;
                                
                                if found_current_above {
                                    for col_index3 in col_start_index2..(col_end_index2) {
                                        new_map[row_index-1][col_index3] = current_char;       
                                    }
                                }
                            }
                            col_index2 += 1;
                        }
                    }
                }
                

            } else {
                new_map[row_index-1][col_index-1] = 9;
                col_index += 1;
                //new_map[row_index-1].push(9);
            }
        }
        println!("new_map is:");
        //for r in new_map.iter() {
        //    println!("{:?}", r);
        //}
        println!("");
        row_index += 1;
    }

    let mut all_text_string = String::with_capacity(new_map.len() * (new_map[0].len()+1));

    for r in new_map.iter() {
        let mut row_string = String::with_capacity(r.len());
        for column in r.iter() {
            row_string.push_str(&column.to_string());
            row_string.push_str(",");
        }
        all_text_string.push_str(&row_string);
        all_text_string.push_str("\n");
    }

    fs::write("output.txt", all_text_string).expect("unable to write to file");


    let mut all_answers: Vec<usize> = vec![0; 2000];

    for row in new_map.iter() {
        for col in row.iter() {
            if *col != 9 {
                all_answers[*col] += 1;
            }
        }
    }

    all_answers.sort_unstable();
    all_answers.reverse();
    println!("{:?}", all_answers);

    println!("largest 3 are {} {} {}", all_answers[0], all_answers[1], all_answers[2]);
    println!("biggest count is {}", (all_answers[0] * all_answers[1] * all_answers[2]));
    // 918944 too big.
    // answer 899392. didnt work. worked in Excel.
}
