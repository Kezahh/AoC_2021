use std::fs;

struct BingoBoard {
    rows: Vec<Vec<i32>>,
    columns: Vec<Vec<i32>>,
}


fn main() {
    let input_filename = "input.txt";
    let input_data = fs::read_to_string(input_filename).expect("unable to read file");
    let split = input_data.lines();
    let input_string_vector: Vec<&str> = split.collect();

    //
    // Get Calling Numbers
    //
    let mut comma_count = 0;
    for c in input_string_vector[0].chars() {
        if c == ',' {
            comma_count += 1;
        }
    }
    let calling_numbers_length = comma_count + 1;
    let mut calling_numbers: Vec<i32> = Vec::with_capacity(calling_numbers_length);

    for num in input_string_vector[0].split(",") {
        calling_numbers.push(num.parse().unwrap());
    }
    
    //
    // Setup Bingo Boards
    //

    let bingo_boards_count = (input_string_vector.len() - 1)/6;
    let mut boards: Vec<BingoBoard> = Vec::with_capacity(bingo_boards_count);

    for i in 0..bingo_boards_count {
        let mut rows: Vec<Vec<i32>> = Vec::with_capacity(5);
        let mut columns: Vec<Vec<i32>> = Vec::with_capacity(5);
        for j in 1..6 {
            let row_string = input_string_vector[1 + (i*6) + j];
            let mut row_nums: Vec<i32> = Vec::with_capacity(5);
            for num in row_string.split(" ") {
                if num != "" {
                    row_nums.push(num.parse().unwrap());
                }
            }
            rows.push(row_nums)
        }

        for j in 0..5 {
            let mut column_nums: Vec<i32> = Vec::with_capacity(5);
            for k in 0..5 {
                column_nums.push(rows[k][j]);
            }
            columns.push(column_nums);
        }
        //println!("{:?}", rows);
        //println!("{:?}", columns);

        let bingo_board = BingoBoard{
            rows: rows,
            columns: columns,
        };
        boards.push(bingo_board);
    }
    
    //
    // Looking for a winner
    //
    let mut current_numbers: Vec<i32> = Vec::with_capacity(calling_numbers_length);
    
    // start by populating first four numbers. Need at least 5 to win
    for i in 0..4 {
        current_numbers.push(calling_numbers[i])
    }

    let mut winning_board_index: i32 = -1;
    let mut boards_have_won: Vec<bool> = vec![false; bingo_boards_count];
    let mut boards_winning_index: Vec<i32> = Vec::with_capacity(bingo_boards_count);
    
    for i in 4..calling_numbers_length {
        current_numbers.push(calling_numbers[i]);

        for (board_index, board) in boards.iter().enumerate() {
            if !boards_have_won[board_index] {
                for row in board.rows.iter() {
                    let mut winner = true;
                    for x in row.iter() {
                        winner = winner && current_numbers.contains(x);
                    }
                    if winner {
                        winning_board_index = board_index as i32;
                        boards_winning_index.push(winning_board_index);
                        boards_have_won[board_index] = true;
                        break;
                    }
                }
                // if won with row, don't check for columns.
                if !boards_have_won[board_index] {
                    for column in board.columns.iter() {
                        let mut winner = true;
                        for x in column.iter() {
                            winner = winner && current_numbers.contains(x);
                        }
                        if winner {
                            winning_board_index = board_index as i32;
                            boards_winning_index.push(winning_board_index);
                            boards_have_won[board_index] = true;
                            break;
                        }
                    }
                }
            }
        }

        let mut all_won = true;
        for x in boards_have_won.iter() {
            all_won = all_won && *x;
        }
        if all_won {
            break;
        }
    }

    //
    // Calculate board score
    //
    winning_board_index = *(boards_winning_index.last().unwrap());      //get last board
    let mut sum_unmarked_numbers = 0;
    for row in &boards[winning_board_index as usize].rows {
        for x in row.iter() {
            if !current_numbers.contains(x) {
                sum_unmarked_numbers += x;
            }
        }
    }
    let winning_score = sum_unmarked_numbers * current_numbers.last().unwrap();

    println!("last number was {:?}", current_numbers.last().unwrap());
    println!("current_numbers is {:?}", current_numbers);
    println!("Winning board index is {:?}", winning_board_index);
    println!("Winning score is {:?}", winning_score);
}
