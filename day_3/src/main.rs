use std::fs;


fn main() {
    let input_filename = "input.txt";
    let input_data = fs::read_to_string(input_filename).expect("unable to read file");
    let split = input_data.lines();
    let input_string_vector: Vec<&str> = split.collect();
    let binary_char_count = input_string_vector[0].len();
    let inputs_count = input_string_vector.len();

    let mut count_ones = vec![0; binary_char_count];

    for x in input_string_vector.iter() {
        for (i, c) in x.chars().enumerate() {
            if c == '1' {
                count_ones[i] += 1
            }
        }
    }

    let mut gamma_rate_string: Vec<char> = Vec::with_capacity(binary_char_count);
    let mut epsilon_rate_string: Vec<char> = Vec::with_capacity(binary_char_count);

    for i in 0..binary_char_count {
        let x = count_ones[i];
        if x > (inputs_count / 2) {
            gamma_rate_string.push('1');
            epsilon_rate_string.push('0');
        } else {
            gamma_rate_string.push('0');
            epsilon_rate_string.push('1');
        }
    }

    let mut gamma_rate: i32 = 0;
    let mut epsilon_rate: i32 = 0;

    for i in 0..binary_char_count {
        let exp: u32 = 11 - i as u32;

        let x = (gamma_rate_string[i] as i32) - 48;    //48 is ascii for '0'
        gamma_rate += x * i32::pow(2, exp);

        let x = (epsilon_rate_string[i] as i32) - 48;  //48 is ascii for '0'
        epsilon_rate += x * i32::pow(2, exp);
    }

    println!("gamma_rate = {:?}, epsilon_rate = {:?}", gamma_rate_string, epsilon_rate_string);
    println!("gamma_rate = {:?}, epsilon_rate = {:?}", gamma_rate, epsilon_rate);
    println!("PART_1 product = {}", gamma_rate * epsilon_rate);

    let target_numbers = input_string_vector.clone();
    let oxygen_index = filter_numbers_by_most_common(&target_numbers, false);
    let co2_index = filter_numbers_by_most_common(&target_numbers, true);

    let oxygen_string = target_numbers[oxygen_index];
    let co2_string = target_numbers[co2_index];

    let oxygen_rating = binary_string_to_dec(oxygen_string);
    let co2_rating = binary_string_to_dec(co2_string);

    println!("oxygen_index = {} and CO2_index = {}", oxygen_index, co2_index);
    println!("oxygen_bin = {} and CO2_bin = {}", oxygen_string, co2_string);
    println!("oxygen = {} and CO2 = {}", oxygen_rating, co2_rating);
    println!("product = {}", oxygen_rating * co2_rating);
}

fn get_most_occuring(input_numbers: &Vec<&str>, char_index: usize, numbers_are_good: Option<&Vec<bool>>)  -> char {
    let mut count_1 = 0;
    let mut count_good_numbers = 0;
    let numbers_count = input_numbers.len();
    let default_good_vector = vec![true; numbers_count];

    let numbers_good_vector = numbers_are_good.unwrap_or(&default_good_vector);

    for (i, x) in input_numbers.iter().enumerate() {
        if numbers_good_vector[i] {
            count_good_numbers += 1;
            println!("{:?}  | target bit {}", x, char_index);
            if x.chars().nth(char_index).unwrap() == '1' {
                count_1 += 1;
            }
        }
    }

    println!("count_1 = {}, count_good_numbers = {}, count_good_numbers/2 = {:?}", count_1, count_good_numbers, count_good_numbers/2);

    // trouble with integer divide. Rather than dividing sum by 2, multiply count by 2.
    if (count_1*2) >= count_good_numbers {
        return '1';
    } else {
        return '0';
    }
}

fn filter_numbers_by_most_common(input_numbers: &Vec<&str>, reverse: bool) -> usize {
    let binary_char_count = input_numbers[0].len();
    let mut numbers_are_good = vec![true; input_numbers.len()];

    for i in 0..binary_char_count {
        let most_common = get_most_occuring(&input_numbers, i, Some(&numbers_are_good));
        if !reverse {
            println!("i = {}, most_common = {}", i, most_common);
        }
        let mut false_set_count = 0;
        for (j, x) in input_numbers.iter().enumerate() {
            if numbers_are_good[j]{
                if reverse {
                    if x.chars().nth(i).unwrap() == most_common {
                        numbers_are_good[j] = false;
                        false_set_count += 1;
                    }
                } else {
                    if x.chars().nth(i).unwrap() != most_common {
                        numbers_are_good[j] = false;
                        false_set_count += 1;
                    }
                }
            }
            let mut count = 0;
            for x in numbers_are_good.iter() {
                if *x {
                    count += 1;
                }
            }
            if count == 1 {
                break;
            }
        }
        if !reverse {
            println!("{}", false_set_count);
        }
    }

    for (i, x) in numbers_are_good.iter().enumerate() {
        if *x {
            return i;
        }
    }

    return 0;
}


fn binary_string_to_dec(input_string: &str) -> i32 {
    let string_length = input_string.len();
    let mut dec_number = 0;

    for (i, x) in input_string.chars().enumerate() {
        let exp: u32 = (string_length - 1 - i) as u32;
        let x = (x as i32) - 48; //48 is ascii for '0'

        dec_number += x * i32::pow(2, exp);
    }

    return dec_number;
}