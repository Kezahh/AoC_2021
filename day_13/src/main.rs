#[path = "../../generic/generic.rs"] mod generic;

fn main() {
    println!("Hello, world!");
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input_file_lines = generic::read_in_file("input.txt");
        let mut points: Vec<Vec<usize>> = Vec::new();
        let mut folds: Vec<Vec<String>> = Vec::new();
        
        let mut max_x = 0;
        let mut max_y = 0;

        for line in input_file_lines.iter() {
            if line.contains(",") {
                let point: Vec<usize> = line.split(",").map(|x| x.parse::<usize>().unwrap()).collect();
                if point[0] > max_x {
                    max_x = point[0];
                }
                if point[1] > max_y {
                    max_y = point[1];
                }
                points.push(point);
            } else if line.contains("fold along") {
                let line: Vec<String> = line.split(" ").map(|x| x.to_string()).collect();
                let line = &line[line.len()-1];
                let fold: Vec<String> = line.split("=").map(|x| x.to_string()).collect();
                folds.push(fold);
            }
        }
        let mut points_map: Vec<Vec<char>> = vec![vec!['.'; max_x+1]; max_y+1];

        for p in points.iter() {
            points_map[p[1]][p[0]] = '#';
        }

        //print_map(&points_map);
        //println!("max_x = {}, max_y = {}", max_x, max_y);

        println!("There are {} points", count_points(&points_map));
        for fold in folds.iter() {
            if fold[0] == "x".to_string() {
                assert!(fold[1].parse::<usize>().unwrap() == points_map[0].len()/2);
                let mut new_map: Vec<Vec<char>> = vec![vec!['.'; points_map[0].len()/2]; points_map.len()];
                for row_index in 0..points_map.len() {
                    for col_index in 0..(points_map[0].len()/2) {
                        //println!("row_index = {}, col_index = {}", row_index, col_index);
                        if points_map[row_index][col_index] == '#' || points_map[row_index][points_map[0].len()-col_index-1] == '#' {
                            new_map[row_index][col_index] = '#';
                        } else {
                            new_map[row_index][col_index] = '.';
                        }
                    }
                }
                points_map = new_map.clone();
            } else if fold[0] == "y".to_string() {
                assert!(fold[1].parse::<usize>().unwrap() == points_map.len()/2);
                let mut new_map: Vec<Vec<char>> = vec![vec!['.'; points_map[0].len()]; points_map.len()/2];
                for row_index in 0..(points_map.len()/2) {
                    for col_index in 0..points_map[0].len() {
                        //println!("row_index = {}, col_index = {}", row_index, col_index);
                        if points_map[row_index][col_index] == '#' || points_map[points_map.len() - row_index - 1][col_index] == '#' {
                            new_map[row_index][col_index] = '#';
                        } else {
                            new_map[row_index][col_index] = '.';
                        }
                    }
                }
                points_map = new_map.clone();
            }

            println!("fold {:?}", fold);
            println!("There are {} points", count_points(&points_map));
            
        }
        print_map(&points_map);

        //println!("{:?}", points_map);
        
    }
}

fn print_map(map: &Vec<Vec<char>>) {
    for row_index in 0..map.len() {
        for col_index in 0..map[0].len() {
            print!("{}", map[row_index][col_index]);
        }
        print!("\n");
    }
    print!("\n");
}

fn count_points(map: &Vec<Vec<char>>) -> usize {
    let mut count = 0;
    for row_index in 0..map.len() {
        for col_index in 0..map[0].len() {
            if map[row_index][col_index] == '#' {
                count += 1;
            }
        }
    }

    return count;
}