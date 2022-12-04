#[path = "../../generic/generic.rs"] mod generic;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::Add;
use std::ops::Sub;
//use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

/// Rotate Axes
///
/// 
///     |  x |  y |  z        |  x |  y |  z        |  x |  y |  z        |  x |  y |  z      
///  x8 |  0 |  1 |  0     x5 |  0 |  1 |  0     x6 |  0 |  1 |  0     x7 |  0 |  1 |  0      
///  y8 |  0 |  0 |  1     y5 | -1 |  0 |  0     y6 |  0 |  0 | -1     y7 |  1 |  0 |  0      
///  z8 |  1 |  0 |  0     z5 |  0 |  0 |  1     z6 | -1 |  0 |  0     z7 |  0 |  0 | -1      
/// 
/// 
///     |  x |  y |  z        |  x |  y |  z        |  x |  y |  z        |  x |  y |  z     
///  x1 |  1 |  0 |  0     x4 |  0 |  0 |  1     x3 | -1 |  0 |  0     x2 |  0 |  0 | -1     
///  y1 |  0 |  1 |  0     y4 |  0 |  1 |  0     y3 |  0 |  1 |  0     y2 |  0 |  1 |  0     
///  z1 |  0 |  0 |  1     z4 | -1 |  0 |  0     z3 |  0 |  0 | -1     z2 |  1 |  0 |  0     
/// 
/// 
///     |  x |  y |  z        |  x |  y |  z        |  x |  y |  z        |  x |  y |  z    
/// x18 |  1 |  0 |  0    x19 |  0 |  0 |  1    x20 | -1 |  0 |  0    x17 |  0 |  0 | -1    
/// y18 |  0 |  0 |  1    y19 | -1 |  0 |  0    y20 |  0 |  0 | -1    y17 |  1 |  0 |  0    
/// z18 |  0 |  1 |  0    z19 |  0 |  1 |  0    z20 |  0 |  1 |  0    z17 |  0 |  1 |  0    
/// 
/// 
///     |  x |  y |  z        |  x |  y |  z        |  x |  y |  z        |  x |  y |  z
/// x13 |  0 | -1 |  0    x14 |  0 | -1 |  0    x15 |  0 | -1 |  0    x16 |  0 | -1 |  0
/// y13 |  1 |  0 |  0    y14 |  0 |  0 |  1    y15 | -1 |  0 |  0    y16 |  0 |  0 | -1
/// z13 |  0 |  0 |  1    z14 | -1 |  0 |  0    z15 |  0 |  0 | -1    z16 |  1 |  0 |  0
/// 
/// 
///     |  x |  y |  z        |  x |  y |  z        |  x |  y |  z        |  x |  y |  z   
/// x12 |  0 |  0 |  1     x9 | -1 |  0 |  0    x10 |  0 |  0 | -1    x11 |  1 |  0 |  0   
/// y12 |  0 | -1 |  0     y9 |  0 | -1 |  0    y10 |  0 | -1 |  0    y11 |  0 | -1 |  0   
/// z12 |  1 |  0 |  0     z9 |  0 |  0 |  1    z10 | -1 |  0 |  0    z11 |  0 |  0 | -1   
/// 
/// 
///     |  x |  y |  z        |  x |  y |  z        |  x |  y |  z        |  x |  y |  z    
/// x22 |  1 |  0 |  0    x23 |  0 |  0 |  1    x24 | -1 |  0 |  0    x21 |  0 |  0 | -1    
/// y22 |  0 |  0 |  1    y23 | -1 |  0 |  0    y24 |  0 |  0 | -1    y21 |  1 |  0 |  0    
/// z22 |  0 | -1 |  0    z23 |  0 | -1 |  0    z24 |  0 | -1 |  0    z21 |  0 | -1 |  0    


impl Point {
    fn from_string(input_string: String) -> Point {
        let split_string: Vec<i32> = input_string.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
        return Point{
            x: split_string[0],
            y: split_string[1],
            z: split_string[2],
        }
    }

    fn from_vector(input_vector: Vec<i32>) -> Point {
        return Point {
            x: input_vector[0],
            y: input_vector[1],
            z: input_vector[2],
        }
    }

    fn zero() -> Point {
        Point {
            x: 0,
            y: 0,
            z: 0,
        }
    }

    fn to_vector(&self) -> Vec<i32> {
        return vec![self.x, self.y, self.z];
    }

    fn get_manhattan_distance(&self) -> usize {
        return (self.x.abs() + self.y.abs() + self.z.abs()) as usize
    }

    fn get_manhattan_to_point(&self, other: Self) -> usize {
        return ((self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()) as usize;
    }

    fn transform(&self, index: usize) -> Point {
        // 24 possible transformations.
        // 6 possible start positions x 4 possible rolls.
        //
        let start_index = index/4;
        let mut current_pos = self.to_vector();
        current_pos = start_position(current_pos.clone(), start_index);

        //println!("start_index = {}, roll_count = {}", start_index, index % 4);
        
        for i in 0..(index % 4) {
            current_pos = roll(current_pos);
        }

        return Point::from_vector(current_pos);
    }

    fn print_straight_vector(&self) {
        println!("{},{},{}", self.x, self.y, self.z);
    }

    fn get_offset(&self, other_point: Point) -> Vec<i32> {
        return vec![
            self.x - other_point.x,
            self.y - other_point.y,
            self.z - other_point.z
        ];
    }

    fn add_offset(&self, offset: Vec<i32>) -> Point {
        return Point {
            x: self.x + offset[0],
            y: self.y + offset[1],
            z: self.z + offset[2],
        };
    }

    fn subtract_offset(&self, offset: Vec<i32>) -> Point {
        return Point {
            x: self.x - offset[0],
            y: self.y - offset[1],
            z: self.z - offset[2],
        };
    }

    fn flipped(&self) -> Point {
        return Point {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        };
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

#[derive(Debug, Clone)]
struct Scanner {
    index: usize,
    position: Point,
    beacons: Vec<Point>,
}


fn get_scanners(input_lines: Vec<String>) -> Vec<Scanner> {
    let mut output_vec: Vec<Scanner> = Vec::new();
    let mut line_index: usize = 0;
    
    while line_index < input_lines.len() {
        let mut current_line: String = input_lines[line_index].clone();
        let mut scanner_index: usize = 0;
        if current_line != "" {
            if current_line[0..3] == "---".to_string() {
                let split_line: Vec<String> = current_line.split("---").map(|x| x.to_string()).collect();
                current_line = split_line[1].clone();
                current_line = current_line[" scanner ".len()..current_line.len()-1].to_string();
                let scanner_index = current_line.parse::<usize>().unwrap();
                let mut beacons: Vec<Point> = Vec::new();

                line_index += 1;
                while line_index < input_lines.len() && input_lines[line_index] != "" {
                    beacons.push(Point::from_string(input_lines[line_index].clone()));
                    line_index += 1;
                }

                output_vec.push(Scanner { index: scanner_index, position: Point::zero(), beacons: beacons.clone() });
            }
        }

        line_index += 1;
    }

    return output_vec;
}

fn roll(input: Vec<i32>) -> Vec<i32> {
    // Y is up, rotated about Y.
    return vec![input[2], input[1], -input[0]];
}

fn start_position(input: Vec<i32>, position: usize) -> Vec<i32> {
    match position {
        // Z stays up and rotate around Z 4x
        0 => vec![ input[0],  input[1],  input[2]],
        1 => vec![ input[1], -input[0],  input[2]],
        2 => vec![-input[0], -input[1],  input[2]],
        3 => vec![-input[1],  input[0],  input[2]],
        // X stays to the left. Rotate Z forward, then backward.
        4 => vec![ input[0], -input[2],  input[1]],
        5 => vec![ input[0],  input[2], -input[1]],
        //default
        _ => vec![ input[0],  input[1],  input[2]],
    }
}

fn get_offset_between_points(points1: &Vec<Point>, points2: &Vec<Point>) -> (Vec<i32>, usize) {
    let mut all_offsets: HashMap<Vec<i32>, usize> = HashMap::new();

    for point_index1 in 0..points1.len() {
        for point_index2 in 0..points2.len() {
            let offset = points2[point_index2].get_offset(points1[point_index1].clone());
            if all_offsets.contains_key(&offset) {
                *all_offsets.get_mut(&offset).unwrap() += 1;
            } else {
                all_offsets.insert(offset, 1);
            }
        }
    }

    let max_offset: Vec<i32> = all_offsets.iter().max_by(|a, b| a.1.cmp(&b.1)).map(|(k, _v)| k).unwrap().to_vec();
    let max_offset_count: usize = *all_offsets.get(&max_offset).unwrap();

    return (max_offset, max_offset_count);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input_lines = generic::read_in_file("input.txt");
        let mut all_scanners: Vec<Scanner> = get_scanners(input_lines);
        
        let mut target_scanners: Vec<Scanner> = Vec::new();
        target_scanners.push(all_scanners[0].clone());

        while target_scanners.len() > 0 {
            let mut new_scanners: Vec<Scanner> = Vec::new();
            for target_scanner in target_scanners.iter() {
                if target_scanner.index == 0 || target_scanner.position.to_vector() != vec![0,0,0] {
                    for adjacent_scanner_index in 1..all_scanners.len() {
                        let mut adjacent_scanner: &mut Scanner = &mut all_scanners[adjacent_scanner_index];

                        // check if adjacent scanner position already set.
                        if target_scanner.index != adjacent_scanner.index && adjacent_scanner.position.to_vector() == vec![0,0,0] {
                            for transform_index in 0..24 {
                                let adjacent_points: Vec<Point> = adjacent_scanner.beacons.iter().map(|x| x.transform(transform_index)).collect();
                                let (offset, offset_count) = get_offset_between_points(&target_scanner.beacons, &adjacent_points);

                                if offset_count >= 12 {
                                    println!("Scanner {}, matches Scanner {} with offset of {:?} and offset_count of {:?} and transform_index {}", adjacent_scanner_index, target_scanner.index, offset, offset_count, transform_index);
                                    adjacent_scanner.position = Point::from_vector(offset.clone()).flipped();
                                    adjacent_scanner.beacons = adjacent_points.iter().map(|x| x.subtract_offset(offset.clone())).collect();
                                    new_scanners.push(adjacent_scanner.clone());
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            target_scanners = new_scanners.clone();
        }

        for scanner in all_scanners.iter() {
            println!("Scanner {} has position {:?}", scanner.index, scanner.position.to_vector());
        }

        // get a set of all points
        let mut all_points: HashSet<Point> = HashSet::new();
        let mut max_manhattan: usize = 0;
        for scanner in all_scanners.iter() {
            //println!("Scanner {}", scanner.index);
            for point in scanner.beacons.iter() {
                all_points.insert(point.clone());
                //point.print_straight_vector();
            }

            for compare_scanner in all_scanners.iter() {
                let manhattan_distance = scanner.position.get_manhattan_to_point(compare_scanner.position.clone());
                if manhattan_distance > max_manhattan {
                    max_manhattan = manhattan_distance;
                }
            }
        }

        println!("There are {} points.", all_points.len());
        println!("Max Manhattan is {}", max_manhattan);
        //584 is too high

        //449 is right
    }


    #[test]
    fn test_rolls() {
        let mut original: Vec<i32> = vec![10, 20, 40];
        //println!("{:?}", original);
        for i in 0..6 {
            let mut current = start_position(original.clone(), i);
            for j in 0..4 {
                println!("{},{},{}", current[0], current[1], current[2]);
                current = roll(current);
            }
        }

    }

    #[test]
    fn more_roll_tests() {
        let mut original: Point = Point{x: 10, y:20, z:40};

        for i in 0..24 {
            original.transform(i).print_straight_vector();
            //original.print_straight_vector();
        }
    }
    
}