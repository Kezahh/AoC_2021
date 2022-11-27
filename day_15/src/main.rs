#[path = "../../generic/generic.rs"] mod generic;
use std::{collections::HashMap, usize};
use std::fmt;

fn main() {
    println!("Hello, world!");
}
#[derive(Clone)]
struct Point {
    x: usize,
    y: usize,
    value: usize,
    distance_to_start: usize,
    next_point: Vec<usize>,
    next_point_set: bool,
}

impl Point {
    fn set_distance(&mut self, new_distance: usize) {
        self.distance_to_start = new_distance;
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point({}, {}) value = {}", self.x, self.y, self.value)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input_file_lines = generic::read_in_file("input.txt");
        let mut input_map: Vec<Vec<usize>> = Vec::new();

        for line in input_file_lines.iter() {
            input_map.push(line.chars().map(|x| (x as usize) - 48).collect());      //48 is ascii for '0'
        }

        let mut points_map: Vec<Vec<Point>> = Vec::new();
        for row_index in 0..input_map.len() {
            let mut points_row: Vec<Point> = Vec::new();
            for col_index in 0..input_map[0].len() {
                points_row.push(Point{
                    x: col_index,
                    y: row_index,
                    value: input_map[row_index][col_index],
                    distance_to_start: 500,
                    next_point_set: false,
                    next_point: vec![0,0],
                });
            }
            points_map.push(points_row);
        }

        points_map[0][0].distance_to_start = 0;
        calc_distances_djikstra(&mut points_map);
        //calc_distances_djikstra(&mut points_map);
        //calc_distances_djikstra(&mut points_map);
        //calc_distances_djikstra(&mut points_map);
        //calc_distances_djikstra(&mut points_map);
        //calc_distances_djikstra(&mut points_map);

        //print_distances(&points_map);
        println!("Min risk is {:?}", get_distance_to_start(points_map.len() - 1, points_map[0].len() - 1, &points_map));
        // 398 is answer.
    }
}

fn sum_x_steps(steps: usize, start_point: &Point, points_map: &mut Vec<Vec<Point>>) -> usize {
    let mut path: Vec<Point> = Vec::new();
    path.push(start_point.clone());
    let mut current_point: Point = start_point.clone();
    for _step in 0..steps {
        //println!("{:?}", current_point);
        if current_point.y != points_map.len() - 1 || current_point.x != points_map[0].len() - 1 {
            if current_point.x == points_map[0].len() - 1 {
                // can only go down
                current_point = points_map[current_point.y + 1][current_point.x].clone();
                path.push(current_point.clone());
            } else if current_point.y == points_map.len() - 1 {
                // can only go left
                current_point = points_map[current_point.y][current_point.x + 1].clone();
            } else {
                let down_point = points_map[current_point.y + 1][current_point.x].clone();
                let right_point = points_map[current_point.y][current_point.x + 1].clone();
                //println!("down point {:?}", down_point);
                //println!("left_point {:?}", right_point);
                if down_point.value <= right_point.value {
                    //println!("went down");
                    current_point = down_point.clone();
                    path.push(current_point.clone());
                } else {
                    //println!("went right");
                    current_point = right_point.clone();
                    path.push(current_point.clone());
                }
            }
        }
    }


    return sum_path(&path);
}

fn sum_path(path: &Vec<Point>) -> usize {
    let point_values: Vec<usize> = path.iter().map(|x| x.value).collect();
    println!("{:?}", point_values);
    let sum: usize = point_values.iter().sum();
    return sum;
}

fn calc_distances(points_map: &Vec<Vec<Point>>) -> Vec<Vec<usize>> {
    let mut distance_map: Vec<Vec<usize>> = vec![vec![0; points_map[0].len()]; points_map.len()];

    for row_index in 0..points_map.len() {
        for col_index in 0..points_map[0].len() {
            let current_point = points_map[row_index][col_index].clone();
            if row_index == 0 && col_index == 0 {
                distance_map[row_index][col_index] = 0;
            } else if row_index == 0 {
                // go left
                distance_map[row_index][col_index] = distance_map[row_index][col_index - 1] + current_point.value;
            } else if col_index == 0 {
                // go up
                distance_map[row_index][col_index] = distance_map[row_index - 1][col_index] + current_point.value;
            } else {
                let up_distance: usize = distance_map[row_index - 1][col_index];
                let left_distance: usize = distance_map[row_index][col_index - 1];

                if up_distance <= left_distance {
                    distance_map[row_index][col_index] = up_distance + current_point.value;
                } else {
                    distance_map[row_index][col_index] = left_distance + current_point.value;
                }
            }
        }
    }

    return distance_map
}

fn calc_distances_djikstra(points_map: &mut Vec<Vec<Point>>) {
    for row_index in 0..points_map.len() {
        for col_index in 0..points_map[0].len() {
            let mut neighbours: Vec<Vec<i32>> = Vec::new();
            let left_point: Vec<i32> = vec![row_index as i32, (col_index as i32) - 1];
            let right_point: Vec<i32> = vec![row_index as i32, (col_index as i32) + 1];
            let up_point: Vec<i32> = vec![(row_index as i32) - 1, col_index as i32];
            let down_point: Vec<i32> = vec![(row_index as i32) + 1, col_index as i32];

            if row_index != points_map.len() - 1 || col_index != points_map[0].len() - 1 {
                if row_index == 0 && col_index == 0 {
                    neighbours.push(down_point);
                    neighbours.push(right_point);
                } else if row_index == 0 && col_index != 0 {
                    neighbours.push(left_point);
                    neighbours.push(down_point);
                    if col_index != points_map[0].len() - 1 {
                        neighbours.push(right_point);
                    }
                } else if col_index == 0 && row_index != 0 {
                    neighbours.push(up_point);
                    neighbours.push(right_point);
                    if row_index != points_map.len() - 1 {
                        neighbours.push(down_point);
                    }
                } else if row_index == points_map.len() - 1 && col_index == points_map[0].len() - 1 {
                    neighbours.push(up_point);
                    neighbours.push(left_point);
                } else if row_index == points_map.len() - 1 && col_index != points_map[0].len() - 1 {
                    neighbours.push(left_point);
                    neighbours.push(up_point);
                    neighbours.push(right_point);
                } else if col_index == points_map[0].len() - 1  && row_index != points_map.len() - 1 {
                    neighbours.push(up_point);
                    neighbours.push(left_point);
                    neighbours.push(down_point);
                } else {
                    neighbours.push(down_point);
                    neighbours.push(up_point);
                    neighbours.push(right_point);
                    neighbours.push(left_point);
                }

                let current_point = points_map[row_index][col_index].clone();
                for neighbour in neighbours {
                    let current_point_distance = get_distance_to_start(current_point.y, current_point.x, &points_map);
                    let neighbour_distance = get_distance_to_start(neighbour[0] as usize, neighbour[1] as usize, &points_map);
                    let mut neighbour_point = &mut points_map[neighbour[0] as usize][neighbour[1] as usize];

                    if !neighbour_point.next_point_set {
                        neighbour_point.next_point = vec![current_point.y, current_point.x];
                        neighbour_point.next_point_set = true;

                    } else {
                        if current_point_distance + neighbour_point.value < neighbour_distance {
                            neighbour_point.next_point = vec![current_point.y, current_point.x];
                        }
                    }
                }
            }
        }
    }
}

fn print_distances(points_map: &Vec<Vec<Point>>) {
    for row_index in 0..points_map.len() {
        for col_index in 0..points_map[0].len() {
            print!("{:?},", points_map[row_index][col_index].distance_to_start);
        }
        print!("\n");
    }
    print!("\n");
}

fn get_distance_to_start(point_row: usize, point_col: usize, points_map: &Vec<Vec<Point>>) -> usize {
    let mut distance = 0;
    let mut current_point: Point = points_map[point_row][point_col].clone();
    while !(current_point.x == 0 && current_point.y == 0) {
        distance += current_point.value;
        current_point = points_map[current_point.next_point[0]][current_point.next_point[1]].clone();
    }

    return distance;
}