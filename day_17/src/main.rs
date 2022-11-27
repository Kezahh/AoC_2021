fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use crate::run_algo;

    #[test]
    fn example_1() {
        // target area: x=20..30, y=-10..-5

        // after each step, x decreases by 1.
        // after each step, y decreases by 1.
        //
        // set initial velocity x (Vx) and velocity y (Vy)
        // Step 0: x=0,                                         y=0
        // Step 1: x=Vx,                                        y=Vy
        // Step 2: x=Vx + (Vx-1),                               y=Vy + (Vy-1)
        // Step 3: x=Vx + (Vx-1) + (Vx-2),                      y=Vy + (Vy-1) + (Vy-2)
        // Step 4: x=Vx + (Vx-1) + (Vx-2) + (Vx-3),             y=Vy + (Vy-1) + (Vy-2) + (Vy-3)
        // Step 5: x=Vx + (Vx-1) + (Vx-2) + (Vx-3) + (Vx-4),    y=Vy + (Vy-1) + (Vy-2) + (Vy-3) + (Vy-4)
        // .....
        // Step n: x=n*Vx - (0 + 1 + 2 + .. + n-1)              y=n*Vy - (0 + 1 + 2 + .. + n-1)
        // Step n: x=n*Vx - (n^2 - 1)/2                         y=n*Vy - (n^2 - 1)/2
        // Step n: n*Vx - x = n*Vy - y
        //
        // Check boundaries
        // n*Vx - 20 = n*Vy + 10
        // 

        let max_y_distance: i32;
        let velocity_count: usize;
        (velocity_count, max_y_distance) = run_algo(20, 30, -10, -5);
        assert!(max_y_distance == 45);
        assert!(velocity_count == 112);
        println!("{:?}", velocity_count);
    }

    #[test]
    fn part_1() {
        // target area: x=48..70, y=-189..-148
        let max_y_distance: i32;
        let velocity_count: usize;
        (velocity_count, max_y_distance) = run_algo(48, 70, -189, -148);
        assert!(max_y_distance == 17766);
    }

    #[test]
    fn part_2() {
        let max_y_distance: i32;
        let velocity_count: usize;
        (velocity_count, max_y_distance) = run_algo(48, 70, -189, -148);
        assert!(max_y_distance == 17766);
        //assert!(velocity_count == 112);
        println!("{:?}", velocity_count);
    }
}

fn run_algo(min_x: i32, max_x: i32, min_y: i32, max_y: i32) -> (usize, i32) {
    let max_vy: i32 = 1000;
    let max_vx: i32 = 200;
    let max_step: i32 = 2000;

    let mut max_y_distance: i32 = 0;
    let mut max_y_distance_answer: i32 = 0;

    let mut velocity_count = 0;

    for vy in -300..max_vy {
        for vx in 1..max_vx {
            let mut good_velocity = false;
            for step in 1..max_step {
                let y_distance: i32 = step*vy - ((step * step)-step)/2;
                let x_distance: i32;
                let mut x_step_modifier: i32 = step;
                if step > vx {
                    x_step_modifier = vx;
                }
                x_distance = x_step_modifier*vx - ((x_step_modifier * x_step_modifier)-x_step_modifier)/2;

                if y_distance > max_y_distance {
                    max_y_distance = y_distance;
                }

                if x_distance >= min_x && x_distance <= max_x && y_distance >= min_y && y_distance <= max_y {
                    println!("vx = {}, vy = {}, step = {}, x = {}, y = {}, (max_y = {})", vx, vy, step, x_distance, y_distance, max_y_distance);
                    if max_y_distance > max_y_distance_answer {
                        max_y_distance_answer = max_y_distance;
                    }
                    good_velocity = true;
                }
                //println!("vx = {}, vy = {}, step = {}, x = {}, y = {}", vx, vy, step, x_distance, y_distance);
            }
            if good_velocity {
                velocity_count += 1;
            }
        }
    }

    return (velocity_count, max_y_distance_answer);
}