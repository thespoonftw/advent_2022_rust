use std::cmp;

use crate::problem::Problem;
use ndarray::Array2;

pub struct Day14();

impl Problem for Day14 {

    fn part_one(&self, input: &str) -> String {
        let mut sim = SandSim::new(input, false);
        return sim.simulate().to_string();
    }

    fn part_two(&self, input: &str) -> String {
        let mut sim = SandSim::new(input, true);
        return sim.simulate().to_string();
    }
}

struct SandSim {
    max_y: usize,
    arr: Array2<bool>
}

impl SandSim {

    fn new(input: &str, add_ground: bool) -> SandSim {

        // parse data
        let tuples:Vec<Vec<(usize, usize)>> = input.lines()
            .into_iter().map(|line| 
                line.split(" -> ").into_iter().map(|coord| 
                    coord_to_tuple(coord)
                ).collect()
            ).collect();

        // max x/y values
        let flatten:Vec<(usize, usize)> = tuples.clone().into_iter().flatten().collect();
        let max_y = flatten.iter().max_by_key(|t| t.1).unwrap().1 + 3;
        let max_x = 1000;
        
        let arr:Array2<bool> = Array2::from_elem((max_x, max_y), false); 
        let mut sim = SandSim { max_y, arr };

        if add_ground {
            sim.draw_line((0, max_y - 1), (max_x - 1, max_y - 1));
        }

        // draw lines
        for rock_line in tuples {
            for i in 0..rock_line.len()-1 {
                sim.draw_line(rock_line[i], rock_line[i + 1]);
            }
        }
    
        return sim;       
    }

    fn draw_line(&mut self, from: (usize, usize), to: (usize, usize)) {

        let x_from = from.0 as i32;
        let y_from = from.1 as i32;
        let x_to = to.0 as i32;
        let y_to = to.1 as i32;
        let steps = cmp::max((x_from - x_to).abs(), (y_from - y_to).abs());
        let dx = (x_to - x_from).signum();
        let dy = (y_to - y_from).signum();
    
        for i in 0..steps+1 {
            let x = x_from + (i * dx);
            let y = y_from + (i * dy);
            self.arr[[x as usize, y as usize]] = true;
        }
    }

    fn simulate(&mut self) -> usize {

        let mut sand_count = 0;
        while self.simulate_grain() {
            sand_count += 1;
        }
        return sand_count;
    }

    fn simulate_grain(&mut self) -> bool {

        let mut x = 500;
        let mut y = 0;

        if self.arr[[x, y]] == true {
            return false; // start is blocked
        }
    
        loop {
    
            if y + 1 >= self.max_y {
                return false; // fell into the abyss
            }
            else if self.arr[[x, y + 1]] == false {
                y += 1;
            }
            else if self.arr[[x - 1, y + 1]] == false {
                x -= 1;
                y += 1;
            }
            else if self.arr[[x + 1, y + 1]] == false {
                x += 1;
                y += 1;
            }
            else {
                self.arr[[x, y]] = true;
                return true; // came to rest
            }
        }
    }
    
}



fn coord_to_tuple(input: &str) -> (usize, usize) {
    let mut split = input.split(',');
    let v1 = split.next().unwrap().parse().unwrap();
    let v2 = split.next().unwrap().parse().unwrap();
    return (v1, v2);
}

