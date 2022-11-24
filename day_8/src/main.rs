use std::fs;

fn main() {
    let input_filename = "input.txt";
    let input_data = fs::read_to_string(input_filename).expect("unable to read file");
    let split = input_data.lines();
    let mut input_string_vector: Vec<Vec<&str>> = Vec::new();
    for line in split {
        let split2 = line.split(" ");
        input_string_vector.push(split2.collect());
    }
    
    //println!("{:?}", input_string_vector);

    let unique_lengths: Vec<usize> = vec![2, 3, 4, 7];

    let mut count: usize = 0;
    for string_line in input_string_vector.iter() {
        for i in 0..4 {
            let string_length: usize = string_line[string_line.len()-i-1].len();
            if unique_lengths.contains(&string_length) {
                count += 1;
            }
        }
    }

    //println!("{} unique numbers", count);

    //
    // Sort strings
    //

    let mut clock_lines: Vec<Vec<Vec<char>>> = vec![Vec::new(); input_string_vector.len()];
    for i in 0..input_string_vector.len() {
        for j in 0..input_string_vector[i].len() {
            let mut char_vector: Vec<char> = input_string_vector[i][j].chars().collect();
            char_vector.sort_unstable();
            //println!("{:?}", char_vector.connect(","));
            clock_lines[i].push(char_vector);

        }
    }


    let mut number_answers = Vec::new();

    for clock_line in clock_lines.iter() {
        let mut actual_numbers = vec![Vec::new(); 10];
        for i in 0..10 {
            let line_length = clock_line[i].len();
            if line_length == 2 {
                actual_numbers[1] = clock_line[i].clone();
            } else if line_length == 3 {
                actual_numbers[7] = clock_line[i].clone();
            } else if line_length == 4 {
                actual_numbers[4] = clock_line[i].clone();
            } else if line_length == 7 {
                actual_numbers[8] = clock_line[i].clone();
            }
        }
        
        for i in 0..10 {
            let line_length = clock_line[i].len();
            if line_length == 6 {
                let mut all_in: bool = true;
                for c in actual_numbers[4].iter() {
                    all_in = all_in && clock_line[i].contains(&c);
                }
                if all_in {
                    actual_numbers[9] = clock_line[i].clone();
                } else {
                    all_in = true;
                    for c in actual_numbers[7].iter() {
                        all_in = all_in && clock_line[i].contains(&c);
                    }
                    if all_in {
                        actual_numbers[0] = clock_line[i].clone();
                    } else {
                        actual_numbers[6] = clock_line[i].clone();
                    }
                }
            }
        }

        for i in 0..10 {
            let line_length = clock_line[i].len();
            if line_length == 5 {
                let mut all_in: bool = true;
                for c in actual_numbers[7].iter() {
                    all_in = all_in && clock_line[i].contains(&c);
                }
                if all_in {
                    actual_numbers[3] = clock_line[i].clone();
                } else {
                    // either "2" or "5".
                    // what does 7 have that 6 does not.
                    let mut target_char: char = 'x';
                    //println!("i = {}, chars in 7 are {:?}", i, actual_numbers[7]);
                    //println!("i = {}, chars in 6 are {:?}", i, actual_numbers[6]);
                    for c in actual_numbers[7].iter() {
                        //println!("Checking if {} exists in 6", c);
                        if !actual_numbers[6].contains(&c) {
                            target_char = *c;
                            //println!("{} does not exist in 6", c);
                            break;
                        }
                    }
                    if target_char == 'x' {
                        println!("Very bad");
                    } else {
                        //println!("i = {}, target_char is {}", i, target_char);
                    }
                    // target char is top-right.
                    // check if target char is in digit
                    // if it exists, digit is 2.
                    //println!("i = {}, clock_line[i] = {:?}", i, clock_line[i]);
                    //println!("i = {}, clock_line[i] ({:?}) contains {} = {:?}", i, clock_line[i], target_char, clock_line[i].contains(&target_char));
                    if clock_line[i].contains(&target_char) {
                        actual_numbers[2] = clock_line[i].clone();
                    } else {
                        actual_numbers[5] = clock_line[i].clone();
                    }
                    //println!("{:?}", actual_numbers);
                }
            }
        }
        
        let mut output_numbers = vec![0; 4];
        for i in 0..4 {
            let output_string = clock_line[clock_line.len() -1 - (3-i)].clone();
            for j in 0..10 {
                if output_string == actual_numbers[j] {
                    output_numbers[i] = j;
                }
            }
        }

        println!("{:?}", output_numbers);
        number_answers.push(output_numbers);
        
        //println!("{:?}", actual_numbers);
        //println!("{:?}", clock_line);
    }

    let mut number_sum = 0;
    for x in number_answers.iter() {
        number_sum += x[0] * 1000 + x[1] * 100 + x[2] * 10 + x[3];
    }
    println!("Number sum is {:?}", number_sum);

    // "1" = length 2
    // "7" = length 3
    // "4" = length 4
    // "8" = length 7
    //
    // "0" - has length 6 and matches "7" but not match "4"
    // "2" - has length 5
    // "3" - has length 5 and matches "7"
    // "5" - has length 5
    // "6" - has length 6 and not match "7" and not match "4".
    // "9" - has length 6 and matches "4".
    //
    // 'aaaa' = "7" but not "1"
    //
    


}
