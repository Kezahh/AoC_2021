#[path = "../../generic/generic.rs"] mod generic;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let input_file_lines = generic::read_in_file("input.txt");
    
    
    println!("{:?}", input_file_lines);
}


fn is_lowercase(input_string: String) -> bool {
    let mut all_lowercase = true;
    for c in input_string.chars() {
        let ascii_code = c as i32;
        all_lowercase = all_lowercase && ((ascii_code > 96) && (ascii_code < 123));
    }

    return all_lowercase;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input_file_lines = generic::read_in_file("input.txt");
        let input_commands: Vec<Vec<String>> = parse_input_lines(input_file_lines);
        let mut commands_map = HashMap::new();
        for command in input_commands.iter() {
            commands_map.insert(command[0].clone(), Vec::new());
            commands_map.insert(command[1].clone(), Vec::new());
        }

        for command in input_commands.iter() {
            if command[1] != "start" {
                commands_map.get_mut(&command[0]).unwrap().push(command[1].clone());
            }
            if command[0] != "start" {
                commands_map.get_mut(&command[1]).unwrap().push(command[0].clone())
            }
            
        }

        // for k in commands_map.keys() {
        //     println!("{}: {:?}", k, commands_map.get(k).unwrap());
        // }

        let mut paths: Vec<Vec<String>> = Vec::new();
        for point in commands_map.get("start").unwrap().iter() {
            paths.push(vec!["start".to_string(), point.clone()]);
        }

        // let mut new_paths = Vec::new();
        // for path in paths.iter() {
        //     for point in commands_map.get(&path[path.len()-1]).unwrap().iter() {
        //         let mut new_path = path.clone();
        //         new_path.push(point.to_string());
        //         new_paths.push(new_path.clone())
        //     }
        // }
        // paths = new_paths.clone();


        for _i in 0..40 {
            println!("Round {}", _i);
            let mut new_paths = Vec::new();
            for path in paths.iter() {
                let last_point = &path[path.len() - 1];
                //println!("path is {:?}", path);
                //println!("last point is {:?}", last_point);
                let mut added_point = false;
                if *last_point != "end".to_string() {
                    for point in commands_map.get(&path[path.len() - 1]).unwrap().iter() {
                        if !(is_lowercase(point.to_string()) && path.contains(point)) {
                            let mut new_path = path.clone();
                            new_path.push(point.to_string());
                            new_paths.push(new_path.clone());
                            added_point = true;
                        } else {
                            let mut has_duplicates = false;
                            let mut duplicate_check = Vec::new();
                            for p in path.iter() {
                                if is_lowercase(p.clone()) && duplicate_check.contains(p) {
                                    has_duplicates = true;
                                    break;
                                } else {
                                    duplicate_check.push(p.clone())
                                }
                            }
                            if !has_duplicates {
                                let mut new_path = path.clone();
                                new_path.push(point.to_string());
                                new_paths.push(new_path.clone());
                                added_point = true;
                            }
                        }
                    }
                }
                if !added_point {
                    new_paths.push(path.clone());
                }
            }
            paths = new_paths.clone();
        }

        let mut good_paths = Vec::new();
        for path in paths.iter() {
            if path[path.len()-1] == "end".to_string() {
                good_paths.push(path);
            }
        }

        for p in good_paths.iter() {
            println!("{:?}", p);
        }
        println!("There are {} paths.", good_paths.len());

        //4347 is too low
    }

    #[test]
    fn testing_lowercase() {
        assert!(is_lowercase("c".to_string()));
        assert_ne!(is_lowercase("C".to_string()), true);
    }
}

fn parse_input_lines(input_lines: Vec<String>) -> Vec<Vec<String>> {
    let mut parsed_lines: Vec<Vec<String>> = Vec::new();
    for line in input_lines {
        parsed_lines.push(line.split("-").map(str::to_string).collect());
    }

    return parsed_lines;
}