#[path = "../../generic/generic.rs"] mod generic;
use std::{collections::HashMap, usize};

fn main() {
    println!("Hello, world!");
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input_file_lines = generic::read_in_file("input.txt");
        let mut input_template: String = input_file_lines[0].clone();
        let mut pair_insertion_rules = HashMap::new();
        let mut pair_results_after_1 = HashMap::new();

        for line in input_file_lines.iter() {
            if line.contains("->") {
                let line_split: Vec<&str> = line.split(" -> ").collect();
                pair_insertion_rules.insert(line_split[0], line_split[1]);
                let pair_chars: Vec<char> = line_split[0].chars().collect();
                let result_char: char = line_split[1].chars().next().unwrap();
                let pair_results: Vec<String> = vec![String::from_iter(vec![pair_chars[0], result_char]), String::from_iter(vec![result_char, pair_chars[1]])];
                let pair: String = line_split[0].to_string();
                pair_results_after_1.insert(pair, pair_results);
            }
        }

        println!("begin");
        println!("{:?}", input_template);
        println!("");


        let mut current_pairs = get_pairs_hashmap(&input_template);
        let mut pair_results_after_10 = HashMap::new();
        for pair in pair_insertion_rules.keys() {
            pair_results_after_10.insert(pair, run_calcs(10, pair.to_string(), &pair_insertion_rules));
        }

        for step in 0..40 {
            let mut new_pairs = HashMap::new();
            for pair in current_pairs.keys() {
                let pair_results: &Vec<String> = pair_results_after_1.get(pair).unwrap();
                for pair_result in pair_results.iter() {
                    if new_pairs.contains_key(pair_result) {
                        *new_pairs.get_mut(pair_result).unwrap() += current_pairs.get(pair).unwrap();
                    } else {
                        new_pairs.insert(pair_result.clone(), *current_pairs.get(pair).unwrap());
                    }
                }
            }

            current_pairs = new_pairs.clone();

            println!("After step {}", step+1);
            //println!("{:?}", current_pairs);
            print_pairs_hashmap(&current_pairs);
        }

        let mut char_occurence_hashmap = get_char_occurence_hashmap(&current_pairs);

        // each letter appears twice because of the pair split.
        for value in char_occurence_hashmap.values_mut() {
            *value = *value/2;
        }

        // ad front and end letter as they were not doubled.
        *char_occurence_hashmap.get_mut(&input_template.chars().next().unwrap()).unwrap() += 1;
        *char_occurence_hashmap.get_mut(&input_template.chars().last().unwrap()).unwrap() += 1;

        println!("{:?}", &char_occurence_hashmap);
        let mut sum = 0;
        let mut min_value = 0;
        let mut max_value = 0;
        let mut first_value = true;
        for pair_value in char_occurence_hashmap.values() {
            if first_value {
                first_value = false;
                min_value = *pair_value;
            }

            if *pair_value > max_value {
                max_value = *pair_value;
            }
            if *pair_value < min_value {
                min_value = *pair_value;
            }
            sum += pair_value;
        }

        //println!("{:?}", current_pairs);
        //run_calcs(10, input_template, &pair_insertion_rules);
        //println!("polymer_length = {}", input_template.len());
        println!("most_common - least_common = {}", max_value - min_value);
    }

    #[test]
    fn test_2() {
        let my_string = "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB".to_string();
        let my_pairs = get_pairs_hashmap(&my_string);    
        print_pairs_hashmap(&my_pairs);
        println!("{:?}", get_char_occurence_hashmap(&my_pairs));
    }
}

fn print_pairs_hashmap(pair_map: &HashMap<String, usize>) {
    let mut my_pairs: Vec<_> = pair_map.iter().collect();
    my_pairs.sort_by_key(|a| a.0);
    println!("{:?}", my_pairs);
}

fn get_char_occurence_hashmap(pair_map: &HashMap<String, usize>) -> HashMap<char, usize> {
    let mut output_map: HashMap<char, usize> = HashMap::new();
    for (pair, value) in pair_map {
        for c in pair.chars() {
            if output_map.contains_key(&c) {
                *output_map.get_mut(&c).unwrap() += *value;
            } else {
                output_map.insert(c, *value);
            }
        }
    }

    return output_map;
}

fn count_letters(input_string: &String) -> HashMap<String, usize> {
    let mut output_map = HashMap::new();
    for c in input_string.chars() {
        output_map.insert(c.to_string(), 0);
    }

    for c in input_string.chars() {
        *output_map.get_mut(&c.to_string()).unwrap() += 1;
    }

    return output_map;
}

fn get_largest_value(input_map: &HashMap<String, usize>) -> usize {
    let mut max_value = 0;
    for x in input_map.values() {
        if *x > max_value {
            max_value = *x;
        }
    }
    return max_value;
}

fn get_smallest_value(input_map: &HashMap<String, usize>) -> usize {
    let mut min_value = 0;
    for x in input_map.values() {
        if min_value == 0 {
            min_value = *x;
        }
        if *x < min_value {
            min_value = *x;
        }
    }
    return min_value;
}

fn get_pairs_hashmap(input_string: &String) -> HashMap<String, usize> {
    let mut current_pairs: HashMap<String, usize> = HashMap::new();

    let input_chars: Vec<char> = input_string.chars().collect();
    for char_index in 0..(input_string.len()-1) {
        let pair_string = String::from_iter(vec![input_chars[char_index], input_chars[char_index+1]]);
        current_pairs.insert(pair_string, 0);
    }

    for char_index in 0..(input_string.len()-1) {
        let pair_string = String::from_iter(vec![input_chars[char_index], input_chars[char_index+1]]);
        *current_pairs.get_mut(&pair_string).unwrap() += 1;
    }

    return current_pairs;
}

fn run_calcs(steps: usize, input_template: String, pair_insertion_rules: &HashMap<&str, &str>) -> String {
    let mut running_template = input_template;

    for step in 0..steps {
        let mut new_template = String::new();
        let template_chars: Vec<char> = running_template.chars().collect();
        for char_index in 0..(template_chars.len() - 1) {
            let current_pair = String::from_iter(vec![template_chars[char_index], template_chars[char_index+1]]);
            new_template.push(template_chars[char_index]);
            new_template.push(pair_insertion_rules.get(&*current_pair).unwrap().chars().next().unwrap());
        }
        new_template.push(*template_chars.last().unwrap());
        running_template = new_template;

        let letter_count = count_letters(&running_template);

        //println!("After step {}", step + 1);
        println!("{}", running_template);
        //println!("polymer_length = {}", input_template.len());
        //println!("most_common - least_common = {}", get_largest_value(&letter_count) - get_smallest_value(&letter_count));
        //println!("{:?}", letter_count);
        
        println!("");
    }

    return running_template;
}