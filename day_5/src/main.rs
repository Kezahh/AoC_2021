use std::fs;

struct ShipVector {
    x1: usize,
    x2: usize,
    y1: usize,
    y2: usize,
}

fn main() {
    let input_filename = "input.txt";
    let input_data = fs::read_to_string(input_filename).expect("unable to read file");
    let split = input_data.lines();
    let input_string_vector: Vec<&str> = split.collect();
    
    //
    // Get all points
    //
    let mut all_vectors: Vec<Vec<Vec<usize>>> = Vec::with_capacity(input_string_vector.len());
    let mut all_ship_vectors: Vec<ShipVector> = Vec::with_capacity(input_string_vector.len());

    let mut max_value = 0;
    for line in input_string_vector {
        let mut vector: Vec<Vec<usize>> = vec![vec![0; 2], vec![0; 2]];
        for (i, x) in line.split(" -> ").enumerate() {
            for (j, y) in x.split(",").enumerate() {
                vector[i][j] = y.parse().unwrap();
                if vector[i][j] > max_value {
                    max_value = vector[i][j];
                }
            }
        }
        let ship_vector = ShipVector {
            x1: vector[0][0],
            x2: vector[1][0],
            y1: vector[0][1],
            y2: vector[1][1],
        };
        
        all_vectors.push(vector);
        all_ship_vectors.push(ship_vector);
    }
    
    //
    // Create points grid.
    //
    max_value = max_value + 1; // index of 0
    let mut points_grid: Vec<Vec<i32>> = vec![vec![0; max_value as usize]; max_value as usize];

    for v in all_ship_vectors.iter() {
        let real_x1: usize;
        let real_x2: usize;
        let real_y1: usize;
        let real_y2: usize;
        if v.x1 <= v.x2 {
            real_x1 = v.x1;
            real_x2 = v.x2;
        } else {
            real_x1 = v.x2;
            real_x2 = v.x1;
        }

        if v.y1 <= v.y2 {
            real_y1 = v.y1;
            real_y2 = v.y2;
        } else {
            real_y1 = v.y2;
            real_y2 = v.y1;
        }

        // check if straight line
        if real_y1 == real_y2 {
            // horizontal line
            for i in real_x1..(real_x2 + 1) {
                points_grid[real_y1][i] += 1;
            }
        } else if real_x1 == real_x2 {
            // vertical line
            for i in real_y1..(real_y2 + 1) {
                points_grid[i][real_x1] += 1;
            }
        } else {
            // diagonal line
            if v.x1 < v.x2 {
                if v.y1 < v.y2 {
                    for i in 0..(v.x2 - v.x1 + 1) {
                        //println!("{:?} {:?} {:?} {:?} {:?}", i, v.x1, v.y1, v.x2, v.y2);
                        points_grid[v.y1 + i][v.x1 + i] += 1;
                    }
                } else if v.y1 > v.y2 {
                    for i in 0..(v.x2 - v.x1 + 1) {
                        points_grid[v.y1 - i][v.x1 + i] += 1;
                    }
                }
            } else if v.x1 > v.x2 {
                if v.y1 < v.y2 {
                    for i in 0..(v.x1 - v.x2 + 1) {
                        points_grid[v.y1 + i][v.x1 - i] += 1;
                    }
                } else if v.y1 > v.y2 {
                    for i in 0..(v.x1 - v.x2 + 1) {
                        points_grid[v.y1 - i][v.x1 - i] += 1;
                    }
                }
            }
        }
    }

    let mut count_dangerous = 0;
    let mut all_text_string = String::with_capacity(max_value * (max_value+1));
    for row in points_grid.iter() {
        let mut row_string = String::with_capacity(max_value);
        for column in row.iter() {
            if *column == 0 {
                row_string.push_str(".");
            } else {
                row_string.push_str(&column.to_string());
                if *column > 1 {
                    count_dangerous += 1;
                }
            }
        }
        //println!("{}", row_string);
        
        all_text_string.push_str(&row_string);
        all_text_string.push_str("\n");
    }
    fs::write("output.txt", all_text_string).expect("unable to write to file");
    println!("{}", count_dangerous);
    // 4182 is wrong. too low
    // 5053 is wrong. too low
    // 5084 is correct.
    //println!("{:?}", all_vectors);
}
