#[path = "../../generic/generic.rs"] mod generic;
use std::collections::HashMap;
use std::fmt;


fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone, Default)]
struct SnailPair {
    left_is_number: bool,
    right_is_number: bool,
    left_num: usize,
    right_num: usize,
    left_snail_index: usize,
    right_snail_index: usize,
    snail_index: usize,
}


#[derive(Clone, Default)]
struct SnailRef {
    num: usize,
    depth: usize,
    snail_index: usize,
    is_left: bool,
}

impl fmt::Debug for SnailRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.num, self.depth)
    }
}

impl SnailPair {
    fn get_string(&self, all_snail_nums: &Vec<SnailPair>) -> String {
        let mut answer: String = String::new();
        answer.push_str("[");
        if self.left_is_number {
            answer.push_str(self.left_num.to_string().as_str());
        } else {
            answer.push_str(all_snail_nums[self.left_snail_index].get_string(&all_snail_nums).as_str());
        }
        answer.push_str(",");
        if self.right_is_number {
            answer.push_str(self.right_num.to_string().as_str());
        } else {
            answer.push_str(all_snail_nums[self.right_snail_index].get_string(&all_snail_nums).as_str());
        }
        answer.push_str("]");

        return answer;
    }

    fn get_num_list(&self, arg_depth: Option<usize>, all_snail_nums: &Vec<SnailPair>) -> Vec<SnailRef> {
        let mut answer: Vec<SnailRef> = Vec::new();
        let mut depth: usize;
        if arg_depth == None {
            depth = 1;
        } else {
            depth = arg_depth.unwrap() + 1;
        }

        if self.left_is_number {
            answer.push(SnailRef{num: self.left_num, depth: depth, snail_index: self.snail_index, is_left: true});
        } else {
            answer.append(&mut all_snail_nums[self.left_snail_index].get_num_list(Some(depth), &all_snail_nums));
            //answer.append(&mut self.left_snail.as_ref().unwrap().get_num_list(Some(depth)));
        }
        if self.right_is_number {
            answer.push(SnailRef{num: self.right_num, depth: depth, snail_index: self.snail_index, is_left: false});
        } else {
            answer.append(&mut all_snail_nums[self.right_snail_index].get_num_list(Some(depth), &all_snail_nums));
            //answer.append(&mut self.right_snail.as_ref().unwrap().get_num_list(Some(depth)));
        }

        return answer;
    }

    fn calculate_magnitude(&self, all_snail_nums: &Vec<SnailPair>) -> usize {
        let mut answer: usize = 0;
        if self.left_is_number {
            answer += 3 * self.left_num;
        } else {
            answer += 3 * all_snail_nums[self.left_snail_index].calculate_magnitude(&all_snail_nums);
        }
        if self.right_is_number {
            answer += 2 * self.right_num;
        } else {
            answer += 2 * all_snail_nums[self.right_snail_index].calculate_magnitude(&all_snail_nums);
        }

        return answer;
    }
}

fn string_to_list(snail_num: String) -> Vec<Vec<usize>> {
    let mut output_list = Vec::new();
    let mut char_index: usize = 0;
    let mut depth_count: usize = 0;
    while char_index < snail_num.chars().count() {
        let mut c = snail_num.chars().nth(char_index).unwrap();
        if c == '[' {
            depth_count += 1;
        } else if c == ']' {
            depth_count -= 1;
        } else if c == ',' {
            //pass
        } else {
            let start_number_index: usize = char_index;
            char_index += 1;
            c = snail_num.chars().nth(char_index).unwrap();
            let end_chars: Vec<char> = vec![',', ']', '['];
            while !end_chars.contains(&c) {
                char_index += 1;
            }
            let end_number_index: usize = char_index;

            let temp_number: usize = snail_num[start_number_index..end_number_index].to_string().parse::<usize>().unwrap();
            output_list.push(vec![temp_number, depth_count]);
            char_index -= 1;
        }
        char_index += 1;
    }

    return output_list;
}

fn string_to_snail_num(snail_num: &String, mut all_snail_nums: &mut Vec<SnailPair>) -> Option<SnailPair> {
    //println!("Input string = {:?}", snail_num);
    let mut depth_count = 0;
    let mut max_depth = 0;

    for (i, c) in snail_num.chars().enumerate() {
        if c == '[' {
            depth_count += 1;
            if depth_count > max_depth {
                max_depth = depth_count;
            }
        } else if c == ']' {
            depth_count -= 1;
        } else if c == ',' {
            if depth_count == 1 {
                // found the middle
                // index 1 to the left to remove opening bracket.
                let left_side: String = snail_num[1..(i)].to_string();
                // index 2 to the left to remove comma and 1 to the right to remove closing bracket.
                //println!("Slicing {}, i = {}", snail_num, i);
                let right_side: String = snail_num[(i+1)..(snail_num.len()-1)].to_string();

                // check if numbers.
                let left_side_number: bool = !left_side.parse::<usize>().is_err();
                let right_side_number: bool = !right_side.parse::<usize>().is_err();

                let snail_index: usize = all_snail_nums.len();
                let mut new_snails: Vec<SnailPair> = Vec::new();

                if left_side_number && right_side_number {
                    let mut new_snail: SnailPair = SnailPair {
                        left_is_number: true,
                        right_is_number: true,
                        left_num: left_side.parse::<usize>().unwrap(),
                        right_num: right_side.parse::<usize>().unwrap(),
                        left_snail_index: 0,
                        right_snail_index: 0,
                        snail_index: snail_index,
                    };
                    new_snail.snail_index = all_snail_nums.len();
                    all_snail_nums.push(new_snail);
                    return Some(all_snail_nums.last().unwrap().clone());
                } else if left_side_number && !right_side_number {
                    let right_snail = string_to_snail_num(&right_side, &mut all_snail_nums).unwrap();
                    let mut new_snail: SnailPair = SnailPair {
                        left_is_number: true,
                        right_is_number: false,
                        left_num: left_side.parse::<usize>().unwrap(),
                        right_num: 0,
                        left_snail_index: 0,
                        right_snail_index: right_snail.snail_index,
                        snail_index: snail_index,
                    };
                    new_snail.snail_index = all_snail_nums.len();
                    all_snail_nums.push(new_snail);
                    return Some(all_snail_nums.last().unwrap().clone());
                } else if !left_side_number && right_side_number {
                    let left_snail = string_to_snail_num(&left_side, all_snail_nums).unwrap();
                    let mut new_snail: SnailPair = SnailPair {
                        left_is_number: false,
                        right_is_number: true,
                        left_num: 0,
                        right_num: right_side.parse::<usize>().unwrap(),
                        left_snail_index: left_snail.snail_index,
                        right_snail_index: 0,
                        snail_index: snail_index,
                    };
                    new_snail.snail_index = all_snail_nums.len();
                    all_snail_nums.push(new_snail);
                    return Some(all_snail_nums.last().unwrap().clone());
                } else if !left_side_number && !right_side_number {
                    let left_snail = string_to_snail_num(&left_side, all_snail_nums).unwrap();
                    let right_snail = string_to_snail_num(&right_side, all_snail_nums).unwrap();
                    let mut new_snail: SnailPair = SnailPair {
                        left_is_number: false,
                        right_is_number: false,
                        left_num: 0,
                        right_num: 0,
                        left_snail_index: left_snail.snail_index,
                        right_snail_index: right_snail.snail_index,
                        snail_index: snail_index,
                    };
                    new_snail.snail_index = all_snail_nums.len();
                    all_snail_nums.push(new_snail);
                    return Some(all_snail_nums.last().unwrap().clone());
                }
            }
        }
    }
    
    return None;
    //return *SnailPair::default();
    //return max_depth;
}

fn count_commas(s: &String) -> usize {
    let mut comma_count = 0;
    for c in s.chars() {
        if c == ',' {
            comma_count += 1;
        }
    }
    return comma_count;
}

fn reduce_snail_num(snail_num: &SnailPair, mut all_snail_nums: &mut Vec<SnailPair>, break_after_first: bool) {
    //println!("{:?}", snail_num.get_string(&all_snail_nums));
    if do_explosions(snail_num, all_snail_nums, break_after_first) == false && break_after_first {
        do_splits(snail_num, all_snail_nums);
        return ();
    }
    while !break_after_first && do_splits(snail_num, all_snail_nums) {
        do_explosions(snail_num, all_snail_nums, break_after_first);
    }
}

fn do_explosions(snail_num: &SnailPair, all_snail_nums: &mut Vec<SnailPair>, break_after_first: bool) -> bool {
    let mut current_list = snail_num.get_num_list(None, &all_snail_nums);
    let mut num_index = 0;
    let mut snail_exploded_count = 0;
    
    while num_index < current_list.len(){
        let mut snail_has_exploded = false;
        let snail_to_explode = &current_list[num_index];
        let num: usize = snail_to_explode.num;
        let depth: usize = snail_to_explode.depth;

        if depth > 4 {
            assert!(current_list[num_index+1].depth == depth);

            // increase left number
            if num_index > 0 {
                // number exists to the left.
                let target_snail = &current_list[num_index - 1];
                if target_snail.is_left {
                    assert!(all_snail_nums[target_snail.snail_index].left_is_number);
                    all_snail_nums[target_snail.snail_index].left_num += num;
                } else {
                    assert!(all_snail_nums[target_snail.snail_index].right_is_number);
                    all_snail_nums[target_snail.snail_index].right_num += num;
                }
            }

            // increase right number
            if num_index < (current_list.len() - 2) {
                // number exists to the right.
                let target_snail = &current_list[num_index + 2];
                if target_snail.is_left {
                    assert!(all_snail_nums[target_snail.snail_index].left_is_number);
                    all_snail_nums[target_snail.snail_index].left_num += current_list[num_index+1].num;
                } else {
                    assert!(all_snail_nums[target_snail.snail_index].right_is_number);
                    all_snail_nums[target_snail.snail_index].right_num += current_list[num_index+1].num;
                }
            }

            //break reference to exploded snailnum
            for snail_num in all_snail_nums.iter_mut() {
                if !snail_num.left_is_number && snail_num.left_snail_index == snail_to_explode.snail_index {
                    snail_num.left_snail_index = 0;
                    snail_num.left_is_number = true;
                    snail_num.left_num = 0;
                } else if !snail_num.right_is_number && snail_num.right_snail_index == snail_to_explode.snail_index {
                    snail_num.right_snail_index = 0;
                    snail_num.right_is_number = true;
                    snail_num.right_num = 0;
                }
            }
            //add extra num index to skip right num.
            num_index += 1;
            snail_has_exploded = true;
        }

        num_index += 1;

        if snail_has_exploded {
            snail_exploded_count += 1;
            if break_after_first {
                return true;
            } else {
                current_list = snail_num.get_num_list(None, &all_snail_nums);
                num_index = 0;
                //println!("current_list = {:?}", current_list);
            }
        }   
    }
    return snail_exploded_count > 0;
}

fn do_splits(snail_num: &SnailPair, all_snail_nums: &mut Vec<SnailPair>) -> bool {
    let mut current_list = snail_num.get_num_list(None, &all_snail_nums);
    let mut num_index = 0;

    while num_index < current_list.len(){
        let snail_to_split = &current_list[num_index];
        let num: usize = snail_to_split.num;
    
        if num >= 10 {
            let mut new_snail_num = SnailPair {
                left_is_number: true,
                right_is_number: true,
                left_num: num/2,
                right_num: num - (num/2),
                left_snail_index: 0,
                right_snail_index: 0,
                snail_index: 0,
            };
            new_snail_num.snail_index = all_snail_nums.len();
            let new_snail_num_index: usize = new_snail_num.snail_index;
            all_snail_nums.push(new_snail_num);
            
            if snail_to_split.is_left {
                all_snail_nums[snail_to_split.snail_index].left_is_number = false;
                all_snail_nums[snail_to_split.snail_index].left_num = 0;
                all_snail_nums[snail_to_split.snail_index].left_snail_index = new_snail_num_index;
            } else {
                all_snail_nums[snail_to_split.snail_index].right_is_number = false;
                all_snail_nums[snail_to_split.snail_index].right_num = 0;
                all_snail_nums[snail_to_split.snail_index].right_snail_index = new_snail_num_index;
            }
            return true;
        }
        num_index += 1;
    }

    return false;
}


fn quick_reduce_test(input_string: &str, output_string: &str, break_after_first: bool) {
    let snail_string: String = input_string.to_string();
        
    let mut all_snail_nums: Vec<SnailPair> = Vec::new();
    let mut snail_num: SnailPair = string_to_snail_num(&snail_string, &mut all_snail_nums).unwrap();
    let parent_snail_index: usize = all_snail_nums.len() - 1;
    reduce_snail_num(&snail_num, &mut all_snail_nums, break_after_first);
    snail_num = all_snail_nums[parent_snail_index].clone();
    let reduced_string = snail_num.get_string(&all_snail_nums);
    
    println!("--- TEST ---");
    println!("input = {:?}", input_string);
    println!("expected_output = {:?}", output_string);
    println!("actual output = {:?}", reduced_string);
    println!("---------");

    assert!(output_string.to_string() == reduced_string);
}

fn add_list(list_file: &str, output_string: &str) {
    println!("--- TEST ---");
    println!("input = {:?}", list_file);
    println!("expected_output = {:?}", output_string);

    let file_lines = generic::read_in_file(list_file);
    let mut snail_string: String = file_lines[0].clone();
    let adding_lines: Vec<String> = file_lines[1..].to_vec();
        
    let mut all_snail_nums: Vec<SnailPair> = Vec::new();

    for line_index in 0..file_lines.len() {
        if line_index != 0 {
            let mut new_string = String::new();
            new_string.push_str("[");
            new_string.push_str(snail_string.as_str());
            new_string.push_str(",");
            new_string.push_str(file_lines[line_index].as_str());
            new_string.push_str("]");

            snail_string = new_string.clone();
        }

        //println!("{:?}", snail_string);
        let mut snail_num: SnailPair = string_to_snail_num(&snail_string, &mut all_snail_nums).unwrap();
        let parent_snail_index: usize = all_snail_nums.len() - 1;
        reduce_snail_num(&snail_num, &mut all_snail_nums, false);
        snail_num = all_snail_nums[parent_snail_index].clone();
        
        snail_string = snail_num.get_string(&all_snail_nums);
    }

    println!("actual output = {:?}", snail_string);
    println!("---------");

    //let mut snail_num: SnailPair = string_to_snail_num(&snail_string, &mut all_snail_nums).unwrap();
    //println!("snail num = {:?}", snail_num);
    //println!("{:?}", snail_num.get_num_list(None, &all_snail_nums));

    assert!(output_string.to_string() == snail_string);
}

fn check_magnitude(input_string: &str, expected_answer: usize) {
    let snail_string: String = input_string.to_string();
        
    let mut all_snail_nums: Vec<SnailPair> = Vec::new();
    let mut snail_num: SnailPair = string_to_snail_num(&snail_string, &mut all_snail_nums).unwrap();
    let parent_snail_index: usize = all_snail_nums.len() - 1;
    reduce_snail_num(&snail_num, &mut all_snail_nums, false);
    snail_num = all_snail_nums[parent_snail_index].clone();
    let reduced_string = snail_num.get_string(&all_snail_nums);

    let magnitude = snail_num.calculate_magnitude(&all_snail_nums);
    
    println!("--- TEST ---");
    println!("input = {:?}", input_string);
    println!("expected_output = {:?}", expected_answer);
    println!("actual output = {:?}", magnitude);
    println!("---------");

    assert!(expected_answer == magnitude);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reductions() {
        quick_reduce_test("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]", true);
        quick_reduce_test("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]", true);
        quick_reduce_test("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]", true);
        quick_reduce_test("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", true);
        quick_reduce_test("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[7,0]]]]", true);

        quick_reduce_test("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]", "[[[[0,7],4],[7,[[8,4],9]]],[1,1]]", true);
        quick_reduce_test("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]", "[[[[0,7],4],[15,[0,13]]],[1,1]]", true);
        quick_reduce_test("[[[[0,7],4],[15,[0,13]]],[1,1]]", "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]", true);
        quick_reduce_test("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]", "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]", true);
        quick_reduce_test("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]", "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", true);
    }

    #[test]
    fn full_tests() {
        quick_reduce_test("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]", "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", false);
    }

    #[test]
    fn list_tests () {
        add_list("input_example_1.txt", "[[[[1,1],[2,2]],[3,3]],[4,4]]");
        add_list("input_example_2.txt", "[[[[3,0],[5,3]],[4,4]],[5,5]]");
        add_list("input_example_3.txt", "[[[[5,0],[7,4]],[5,5]],[6,6]]");
        add_list("input_example_4.txt", "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
    }

    #[test]
    fn big_list_tests() {
        add_list("input_example_21.txt", "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]");        
        add_list("input_example_22.txt", "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]");        
        add_list("input_example_23.txt", "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]");        
        add_list("input_example_24.txt", "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]");        
        add_list("input_example_25.txt", "[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]");        
        add_list("input_example_26.txt", "[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]");        
        add_list("input_example_27.txt", "[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]");        
        add_list("input_example_28.txt", "[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]");        
        add_list("input_example_29.txt", "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
    }

    #[test]
    fn test_magnitudes() {
        check_magnitude("[[1,2],[[3,4],5]]", 143);
        check_magnitude("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384);
        check_magnitude("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445);
        check_magnitude("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791);
        check_magnitude("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137);
        check_magnitude("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]", 3488);
    }

    #[test]
    fn final_example() {
        add_list("input_example_31.txt", "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]");
        check_magnitude("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]", 4140);
    }

    #[test]
    fn part_1() {
        let list_file = "input.txt";
        println!("--- TEST ---");
        println!("input = {:?}", list_file);

        let file_lines = generic::read_in_file(list_file);
        let mut snail_string: String = file_lines[0].clone();
        let adding_lines: Vec<String> = file_lines[1..].to_vec();
            
        let mut all_snail_nums: Vec<SnailPair> = Vec::new();
        let mut snail_num: SnailPair = string_to_snail_num(&snail_string, &mut all_snail_nums).unwrap();

        for line_index in 0..file_lines.len() {
            if line_index != 0 {
                let mut new_string = String::new();
                new_string.push_str("[");
                new_string.push_str(snail_string.as_str());
                new_string.push_str(",");
                new_string.push_str(file_lines[line_index].as_str());
                new_string.push_str("]");

                snail_string = new_string.clone();
            }

            snail_num = string_to_snail_num(&snail_string, &mut all_snail_nums).unwrap();
            let parent_snail_index: usize = all_snail_nums.len() - 1;
            reduce_snail_num(&snail_num, &mut all_snail_nums, false);
            snail_num = all_snail_nums[parent_snail_index].clone();
            
            snail_string = snail_num.get_string(&all_snail_nums);
        }

        let magnitude = snail_num.calculate_magnitude(&all_snail_nums);

        println!("actual output = {:?}", snail_string);
        println!("actual magnitude = {:?}", magnitude);
        println!("---------");
    }

    #[test]
    fn part_2() {
        //let list_file = "input_example_41.txt";
        let list_file = "input.txt";
        println!("--- TEST ---");
        println!("input = {:?}", list_file);

        let file_lines = generic::read_in_file(list_file);

        let mut all_possible_snail_nums: Vec<String> = Vec::new();

        for line_index in 0..file_lines.len() {
            for line_index2 in 0..file_lines.len() {
                if line_index2 != line_index {
                    let mut new_string = String::new();
                    new_string.push_str("[");
                    new_string.push_str(file_lines[line_index].as_str());
                    new_string.push_str(",");
                    new_string.push_str(file_lines[line_index2].as_str());
                    new_string.push_str("]");
                    all_possible_snail_nums.push(new_string);
                }
            }
        }

        let mut max_magnitude = 0;
        for snail_string in all_possible_snail_nums {
            let mut all_snail_nums: Vec<SnailPair> = Vec::new();
            let mut snail_num: SnailPair = string_to_snail_num(&snail_string, &mut all_snail_nums).unwrap();
            let parent_snail_index: usize = all_snail_nums.len() - 1;
            reduce_snail_num(&snail_num, &mut all_snail_nums, false);
            snail_num = all_snail_nums[parent_snail_index].clone();
            
            let magnitude = snail_num.calculate_magnitude(&all_snail_nums);

            if magnitude > max_magnitude {
                max_magnitude = magnitude;
            }
        }

        println!("Max magnitude is {:?}", max_magnitude);
    }
}