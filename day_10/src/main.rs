use std::fs;
use std::collections::HashMap;

fn main() {
    let input_filename = "input.txt";
    let input_data = fs::read_to_string(input_filename).expect("unable to read file");
    let split = input_data.lines();
    let all_lines: Vec<&str> = split.collect();
    let mut input_map: Vec<Vec<char>> = Vec::new();

    for line in all_lines.iter() {
        let mut row_chars: Vec<char> = Vec::new();
        for target_char in line.chars() {
            row_chars.push(target_char);
        }
        input_map.push(row_chars);
    }
    
    let bracket_map = HashMap::from([
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>')
    ]);

    let scores_map = HashMap::from([
        ('(', 1),
        ('[', 2),
        ('{', 3),
        ('<', 4)
    ]);


    let mut syntax_errors: Vec<char> = Vec::new();
    let mut line_scores: Vec<usize> = Vec::new();
    for row in input_map.iter() {
        let mut pending_chars: Vec<char> = Vec::new();
        let mut bad_row = false;
        for c in row.iter() {
            if bracket_map.contains_key(c) {
                pending_chars.push(*c);
            } else {
                //println!("{:?}", pending_chars);
                let last_char: char = pending_chars[pending_chars.len()-1];
                if *c == *bracket_map.get(&last_char).unwrap() {
                    pending_chars.pop();
                } else {
                    syntax_errors.push(*c);
                    bad_row = true;
                    break;
                }
            }
        }

        if !bad_row {
            let mut line_score = 0;
            pending_chars.reverse();
            for c in pending_chars.iter() {
                line_score = (line_score * 5) + scores_map.get(c).unwrap();
            }
            line_scores.push(line_score);

            println!("pending_chars = {:?}", pending_chars);
            println!("line_score = {:?}", line_score);
        }
    }
    
    let syntax_scores = HashMap::from([
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137)
    ]);
    let mut total_score = 0;
    for c in syntax_errors.iter() {
        total_score += syntax_scores.get(c).unwrap()
    }

    line_scores.sort_unstable();
    let mid_score = line_scores[line_scores.len()/2];

    println!("Syntax Errors = {:?}", syntax_errors);
    println!("Syntax score = {:?}", total_score);

    println!("row_scores = {:?}", line_scores);
    println!("All score = {:?}", mid_score);
    // 20766005979 is too high
}
