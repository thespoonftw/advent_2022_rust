use crate::problem::Problem;
use ndarray::Array2;

pub struct Day09();

impl Problem for Day09 {

    fn part_one(&self, input: &str) -> String {
        let mut x = StringSim::new(input, 2);
        x.simulate();
        return x.sum_visits().to_string();
    }

    fn part_two(&self, input: &str) -> String {
        let mut x = StringSim::new(input, 10);
        x.simulate();
        return x.sum_visits().to_string();
    }
}

#[derive(Debug)]
#[derive(Clone)]
struct Instruction {
    length: usize,
    dir: char,
}

impl Instruction {
    fn new(line: &str) -> Instruction {
        let s = line.split_whitespace().collect::<Vec<&str>>();
        return Instruction { length: s[1].parse().unwrap(), dir: s[0].parse().unwrap() };
    }
}

struct StringSim {
    instructions: Vec<Instruction>,
    visited_array: Array2<bool>,
    segments: Vec<(i32, i32)>,
    segment_length: usize,
}

impl StringSim {

    fn new(input: &str, segment_length: usize) -> StringSim {
        let size = 1000;
        let mid = size as i32 / 2;
        let instructions = input.lines().map(|l| Instruction::new(l)).collect();
        let visited_array = Array2::from_elem((size, size), false); 
        let segments = (0..segment_length).map(|_| (mid, mid)).collect();
        return StringSim{instructions, visited_array, segments, segment_length };
    }

    fn simulate(&mut self) {
        for ins in &self.instructions.clone() {
            if ins.dir == 'U' {
                self.sim_move(0, 1, ins.length);
            } else if ins.dir == 'D' {
                self.sim_move(0, -1, ins.length);
            } else if ins.dir == 'L' {
                self.sim_move(-1, 0, ins.length);
            } else if ins.dir == 'R' {
                self.sim_move(1, 0, ins.length);
            }
        }
    }

    fn sim_move(&mut self, dx: i32, dy: i32, steps: usize) {
        for _ in 0..steps {

            let head= self.segments[0];
            self.segments[0] = (head.0 + dx, head.1 + dy);

            for i in 0..self.segment_length-1 {
                let dx = self.segments[i].0 - self.segments[i+1].0;
                let dy = self.segments[i].1 - self.segments[i+1].1;
                if dx.abs() >= 2 || dy.abs() >= 2 {
                    self.segments[i+1] = (self.segments[i+1].0 + dx.signum(), self.segments[i+1].1 + dy.signum());
                }
            }

            let tail = self.segments[self.segment_length-1];            
            self.visited_array[[tail.0 as usize, tail.1 as usize]] = true;
        }
    }

    fn sum_visits(&self) -> usize {
        return self.visited_array.map(|x| if *x { 1 } else { 0 }).sum();
    }

}