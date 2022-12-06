#[path = "../../generic/generic.rs"] mod generic;
//use std::collections::HashMap;
//use std::collections::HashSet;
//use std::ops::Add;
//use std::ops::Sub;

fn main() {
    println!("Hello, world!");
}

fn run_solution(input_file: &str) -> usize {
    let input_lines: Vec<String> = generic::read_in_file(input_file);
    let mut player_positions: Vec<usize> = input_lines.iter().map(|x| x.split(" ").last().unwrap().parse::<usize>().unwrap()).collect();
    let mut player_scores: Vec<usize> = vec![0; player_positions.len()];

    for (i, player_position) in player_positions.iter().enumerate() {
        println!("Player {} starting position = {}", i + 1, player_position);
    }

    let max_rolls: usize = 100;
    let mut roll_index = 0;

    let roll_count = 3;
    let mut played_roll_count = 0;

    let mut run_index: usize = 0;
    let end_score: usize = 1000;
    while *player_scores.iter().max().unwrap() < end_score {
        run_index = (run_index % player_positions.len());

        let mut added: Vec<usize> = Vec::new();
        for roll in 0..roll_count {
            player_positions[run_index] += (roll_index + 1);
            added.push(roll_index + 1);
            roll_index = (roll_index % 100) + 1;
            played_roll_count += 1;
        }
        println!("Added {:?}", added);

        player_positions[run_index] = player_positions[run_index] % 10;
        if player_positions[run_index] == 0 {
            player_positions[run_index] = 10;
        }

        player_scores[run_index] += player_positions[run_index];

        println!("Player {} position = {}", run_index + 1, player_positions[run_index]);
        run_index += 1;
    }

    let mut product = player_scores.iter().min().unwrap() * played_roll_count;

    println!("Game Ended");
    println!("----------");
    println!("play has rolled {} times.", played_roll_count);
    for (i, player_score) in player_scores.iter().enumerate() {
        println!("Player {} score = {}", i + 1, player_score);
    }
    println!("----------");
    println!("Product is {}", product);
    println!("----------");

    return product;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert!(run_solution("input_example_1.txt") == 739785);
    }

    #[test]
    fn part_1() {
        assert!(run_solution("input.txt") == 925605);
    }

    #[test]
    fn part_2() {
        assert!(run_solution("input.txt") == 19638);
        //21061 is too high
    }
}