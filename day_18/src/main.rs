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
    let mut num_index: usize = 0;
    let mut current_list = snail_num.get_num_list(None, &all_snail_nums);
    let mut all_explosions_done = false;
    let mut not_exploded_count = 0;

    println!("{:?}", snail_num.get_string(&all_snail_nums));
    println!("start current_list = {:?}", current_list);
    while num_index < current_list.len(){
        
        let mut snail_has_exploded = false;
        let snail_to_explode = &current_list[num_index];
        let num: usize = snail_to_explode.num;
        let depth: usize = snail_to_explode.depth;

        if depth > 4 && !all_explosions_done {
            //println!("num_index = {:?}", num_index);
            //println!("current_list = {:?}", current_list);
            assert!(current_list[num_index+1].depth == depth);

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
        } else {
            not_exploded_count += 1;
        }
        if not_exploded_count == current_list.len() {
            all_explosions_done = true;
            num_index = 0;
        }

        if all_explosions_done && num >= 10 {
            println!("just before split");
            println!("current_list = {:?}", current_list);
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
            
            if snail_to_explode.is_left {
                all_snail_nums[snail_to_explode.snail_index].left_is_number = false;
                all_snail_nums[snail_to_explode.snail_index].left_num = 0;
                all_snail_nums[snail_to_explode.snail_index].left_snail_index = new_snail_num_index;
            } else {
                all_snail_nums[snail_to_explode.snail_index].right_is_number = false;
                all_snail_nums[snail_to_explode.snail_index].right_num = 0;
                all_snail_nums[snail_to_explode.snail_index].right_snail_index = new_snail_num_index;
            }
            snail_has_exploded = true;

            // after a split, reset to do explosions again.
            all_explosions_done = false;
            not_exploded_count = 0;
        }
        num_index += 1;

        if snail_has_exploded {
            //println!("{:?}", snail_num.get_string(&all_snail_nums));
            if break_after_first {
                break;
            } else {
                current_list = snail_num.get_num_list(None, &all_snail_nums);
                num_index = 0;
                println!("current_list = {:?}", current_list);
            }
        }
    }
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

        println!("{:?}", snail_string);
        let mut snail_num: SnailPair = string_to_snail_num(&snail_string, &mut all_snail_nums).unwrap();
        let parent_snail_index: usize = all_snail_nums.len() - 1;
        reduce_snail_num(&snail_num, &mut all_snail_nums, false);
        snail_num = all_snail_nums[parent_snail_index].clone();
        
        snail_string = snail_num.get_string(&all_snail_nums);
    }

    
    
    println!("--- TEST ---");
    println!("input = {:?}", list_file);
    println!("expected_output = {:?}", output_string);
    println!("actual output = {:?}", snail_string);
    println!("---------");

    let mut snail_num: SnailPair = string_to_snail_num(&snail_string, &mut all_snail_nums).unwrap();
    println!("snail num = {:?}", snail_num);
    println!("{:?}", snail_num.get_num_list(None, &all_snail_nums));

    assert!(output_string.to_string() == snail_string);
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
        //add_list("input_example_1.txt", "[[[[1,1],[2,2]],[3,3]],[4,4]]");
        //add_list("input_example_2.txt", "[[[[3,0],[5,3]],[4,4]],[5,5]]");
        //add_list("input_example_3.txt", "[[[[5,0],[7,4]],[5,5]],[6,6]]");
        add_list("input_example_4.txt", "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
    }

    #[test]
    fn testing_more() {
        let snail_string: String = "[[1,[1,[4,[5,5]]]],[2,[3,3]]]".to_string();
        
        let mut all_snail_nums: Vec<SnailPair> = Vec::new();
        let snail_num: SnailPair = string_to_snail_num(&snail_string, &mut all_snail_nums).unwrap();
        println!("{:?}", snail_num.get_string(&all_snail_nums));
        reduce_snail_num(&snail_num, &mut all_snail_nums, true);
        
        println!("{:?}", snail_num.get_string(&all_snail_nums));
    }


    fn part_1() {
        let input_lines = generic::read_in_file("input_example_1.txt");
    }
}