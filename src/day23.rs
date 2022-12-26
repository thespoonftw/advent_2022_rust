use crate::problem::Problem;
use ndarray::Array2;

pub struct Day23();

impl Problem for Day23 {

    fn part_one(&self, input: &str) -> String {
        let mut sim = ElfSim::new(input);
        for _ in 0..10 {
            sim.take_step();
        }        
        return sim.calculate_area().to_string();
    }

    fn part_two(&self, input: &str) -> String {
        let mut sim = ElfSim::new(input);
        let mut i = 0;
        while sim.take_step() {
            i += 1;
        }
        return i.to_string();
    }
}

#[derive(Clone, Debug)]
struct Elf {
    x: usize,
    y: usize,
    x_target: usize,
    y_target: usize,
}

#[derive(Clone, Debug)]
struct ElfSim {
    elves: Vec<Elf>,
    grid: Array2<bool>,
    size: usize,    
    direction: usize,
}

impl ElfSim {

    fn new(input: &str) -> ElfSim {

        let size = 1000;
        let mid = size / 2;

        let lines:Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
        let height = lines.len();
        let width = lines[0].len();

        let mut elves = Vec::new();
        let mut grid = Array2::from_elem((size, size), false); 

        for y_index in 0..height {
            let line = lines[y_index].clone();
            for x_index in 0..width {
                if line[x_index] == '#' {
                    let x = x_index + mid;
                    let y = y_index + mid;
                    elves.push(Elf { x, y, x_target: 0, y_target: 0 });
                    grid[(x, y)] = true;
                }            
            }
        }
        return ElfSim { elves, grid, size, direction: 0 };
    }

    fn take_step(&mut self) -> bool {

        let len = self.elves.len();
        let mut new_grid = Array2::from_elem((self.size, self.size), false); 
        let mut target_grid = Array2::from_elem((self.size, self.size), 0); 

        for i in 0..len {
            let target = self.get_elf_target(i);
            let mut elf = self.elves.get_mut(i).unwrap();
            elf.x_target = target.0;
            elf.y_target = target.1;          
            target_grid[target] += 1;
        }

        let mut checker = false;
        for i in 0..len {
            let mut elf = self.elves.get_mut(i).unwrap();
            let target_value = target_grid[(elf.x_target, elf.y_target)];
            if target_value == 1 && (elf.x != elf.x_target || elf.y != elf.y_target) {
                checker = true;
                elf.x = elf.x_target;
                elf.y = elf.y_target;              
            }
            new_grid[(elf.x, elf.y)] = true;
        }

        self.grid = new_grid;
        self.direction = (self.direction + 1) % 4;
        return checker;

    }

    #[allow(dead_code)]
    fn print(&self) {
        for line in self.grid.columns() {
            let v:String = line.iter().map(|b| if *b {'#'} else {'.'}).collect();
            println!("{v}");
        }
    }

    fn get_elf_target(&mut self, index: usize) -> (usize, usize) {
        let elf = self.elves.get(index).unwrap();
        let any_neighbours = ElfSim::get_all_neighbours(elf.x, elf.y).iter().map(|v| self.grid[*v]).any(|b| b);
        if !any_neighbours { return(elf.x, elf.y); }

        for i in 0..4 {
            let dir_neighbours = ElfSim::get_neighbours(elf.x, elf.y, (self.direction + i) % 4);
            let any_in_dir = dir_neighbours.iter().map(|v| self.grid[*v]).any(|b| b);
            if !any_in_dir {
                let target = dir_neighbours[1];
                return target;
            }
        }
        return (elf.x, elf.y);
    }

    fn get_all_neighbours(x: usize, y: usize) -> Vec<(usize, usize)> {
        return vec![
            (x    , y - 1),
            (x - 1, y - 1),
            (x - 1, y    ),
            (x - 1, y + 1),
            (x    , y + 1),
            (x + 1, y + 1),
            (x + 1, y    ),
            (x + 1, y - 1),
        ];
    }

    fn get_neighbours(x: usize, y: usize, dir: usize) -> Vec<(usize, usize)> {
        if dir == 0 {
            return vec![(x-1,y-1),(x,y-1),(x+1,y-1)];
        } else if dir == 1 {
            return vec![(x-1,y+1),(x,y+1),(x+1,y+1)];
        } else if dir == 2 {
            return vec![(x-1,y-1),(x-1,y),(x-1,y+1)];
        } else {
            return vec![(x+1,y-1),(x+1,y),(x+1,y+1)];
        }
    }

    fn calculate_area(&self) -> usize {
        let mut x_min = usize::MAX;
        let mut x_max = usize::MIN;
        let mut y_min = usize::MAX;
        let mut y_max = usize::MIN;
        for elf in self.elves.clone() {
            if elf.x < x_min { x_min = elf.x; }
            if elf.x > x_max { x_max = elf.x; }
            if elf.y < y_min { y_min = elf.y; }
            if elf.y > y_max { y_max = elf.y; }
        }
        return ((y_max - y_min + 1) * (x_max - x_min + 1)) - self.elves.len();
    }
}


