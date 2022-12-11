#[path = "../../generic/generic.rs"] mod generic;
//use std::collections::HashMap;
//use std::collections::HashSet;
//use std::ops::Add;
//use std::ops::Sub;

#[derive(Debug, Clone)]
struct Player {
    position: usize,
    score: usize,
}

impl Player {
    fn new(position: usize) -> Self {
        return Self{position: position, score: 0};
    }

    fn add_roll(&mut self, roll: usize) {
        self.position += roll;
        self.position = self.position % 10;
        if self.position == 0 {
            self.position = 10;
        }

        self.score += self.position;
    }
}


#[derive(Debug,Clone)]
struct Universe {
    current_player_index: usize,
    players: Vec<Player>,
    quantity: usize,
    game_finished: bool,
    winner_index: usize,
}

impl Universe {
    fn new(players: Vec<Player>) -> Self {
        return Self{current_player_index: 0, players: players, quantity: 1, game_finished: false, winner_index: 0};
    }

    fn play_round(&self) -> Vec<Self> {
        let mut output_vec: Vec<Self> = Vec::new();

        // 1x 3
        // 3x 4
        // 6x 5
        // 7x 6
        // 6x 7
        // 3x 8
        // 1x 9

        output_vec.push(self.split_with_roll(3, 1));
        output_vec.push(self.split_with_roll(4, 3));
        output_vec.push(self.split_with_roll(5, 6));
        output_vec.push(self.split_with_roll(6, 7));
        output_vec.push(self.split_with_roll(7, 6));
        output_vec.push(self.split_with_roll(8, 3));
        output_vec.push(self.split_with_roll(9, 1));

        return output_vec;
    }

    fn split_with_roll(&self, roll_size: usize, quantity: usize) -> Self {
        let mut output: Self = self.clone();
        output.quantity = output.quantity * quantity;
        output.players[output.current_player_index].add_roll(roll_size);

        // println!("Player {}, added roll {}", output.current_player_index, roll_size);
        // println!("Player {} has a score {}", output.current_player_index, output.players[output.current_player_index].score);
        if output.players[output.current_player_index].score >= 21 {
            output.game_finished = true;
            output.winner_index = output.current_player_index;
        }

        output.current_player_index += 1;
        output.current_player_index = output.current_player_index % 2;

        return output;
    }
}


fn run_solution(input_file: &str) -> usize {
    let input_lines: Vec<String> = generic::read_in_file(input_file);
    let starting_positions: Vec<usize> = input_lines.iter().map(|x| x.split(" ").last().unwrap().parse::<usize>().unwrap()).collect();
    let mut players: Vec<Player> = Vec::new();

    for position in starting_positions {
        players.push(Player::new(position));
    }

    let max_rolls: usize = 100;
    let mut roll_index = 0;

    let roll_count = 3;
    let mut played_roll_count = 0;

    let mut run_index: usize = 0;
    let end_score: usize = 1000;
    while players.iter().map(|x| x.score).max().unwrap() < end_score {
        run_index = run_index % players.len();

        let mut added_rolls: Vec<usize> = Vec::new();
        for roll in 0..roll_count {
            added_rolls.push(roll_index + 1);
            roll_index = (roll_index % max_rolls) + 1;
            played_roll_count += 1;
        }
        //println!("Added {:?}", added_rolls);
        players[run_index].add_roll(added_rolls.iter().sum());

        //println!("Player {} position = {}", run_index + 1, players[run_index].position);
        //println!("Player {} score = {}", run_index + 1, players[run_index].score);
        run_index += 1;
    }

    let mut product = players.iter().map(|x| x.score).min().unwrap() * played_roll_count;

    println!("Game Ended");
    println!("----------");
    println!("play has rolled {} times.", played_roll_count);
    for (i, player_score) in players.iter().map(|x| x.score).enumerate() {
        println!("Player {} score = {}", i + 1, player_score);
    }
    println!("----------");
    println!("Product is {}", product);
    println!("----------");

    return product;
}

fn run_solution_part2(input_file: &str) -> usize {
    let input_lines: Vec<String> = generic::read_in_file(input_file);
    let starting_positions: Vec<usize> = input_lines.iter().map(|x| x.split(" ").last().unwrap().parse::<usize>().unwrap()).collect();
    let mut players: Vec<Player> = Vec::new();

    for position in starting_positions {
        players.push(Player::new(position));
    }

    let mut roll_index = 0;
    let mut played_roll_count = 0;

    let mut run_index: usize = 0;
    let end_score: usize = 21;

    let mut universes: Vec<Universe> = Vec::new();
    let mut completed_universes: Vec<Universe> = Vec::new();

    universes.push(Universe::new(players));

    for _i in 0..22 {
        let mut new_universes: Vec<Universe> = Vec::new();
        let mut all_finished = true;
        for universe in universes.iter() {
            if !universe.game_finished {
                new_universes.append(&mut universe.play_round());
                all_finished = false;
            } else {
                completed_universes.push(universe.clone());
            }
        }
        println!("added {} new universes, completed {} universes.", new_universes.len(), completed_universes.len());

        universes = new_universes.clone();

        if all_finished {
            break;
        }

        if _i == 100 {
            for (j, universe) in universes.iter().enumerate() {
                println!("{:?}", universe);
                if j == 1000 {
                    break;
                }
            }
        }
    }


    let mut win_count: Vec<usize> = vec![0, 0];

    for universe in completed_universes.iter() {
        win_count[universe.winner_index] += universe.quantity;
    }


    // for universe in universes {
    //     println!("{:?}", universe);
    // }

    let max_score: usize = *win_count.iter().max().unwrap();
    println!("Max score is {}", max_score);
    return max_score;
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
    fn example_2() {
        assert!(run_solution_part2("input_example_1.txt") == 444356092776315);
    }

    #[test]
    fn part_2() {
        assert!(run_solution_part2("input.txt") == 486638407378784);
    }
}