use crate::problem::Problem;
use ndarray::Array2;

pub struct Day22();

impl Problem for Day22 {

    fn part_one(&self, input: &str) -> String {
        let mut map = MapState::new(input);
        map.navigate();
        return map.get_password().to_string();
    }

    fn part_two(&self, input: &str) -> String {
        return input.len().to_string();
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Tile {
    Void,
    Path,
    Wall,
}

#[derive(Debug)]
struct MapState {
    map: Map,
    instructions: Vec<String>,
    x: usize,
    y: usize,
    facing: usize,
}

#[derive(Debug)]
struct Map {
    grid: Array2<Tile>,
    row_limits: Vec<(usize, usize)>,
    col_limits: Vec<(usize, usize)>,
}

impl Map {

    fn new(lines: Vec<&str>) -> Map {

        let width = lines.iter().max_by_key(|l| l.len()).unwrap().len();
        let height = lines.len();

        let mut grid = Array2::from_elem((height, width), Tile::Void); 
        for y in 0..height {
            let line = lines[y];
            for (x, c) in line.chars().enumerate() {
                if c == '.' {
                    grid[(y,x)] = Tile::Path; // y-x for columns-rows
                } else if c == '#' {
                    grid[(y,x)] = Tile::Wall;
                }
            }
        }
        
        let mut col_limits = Vec::new();
        for x in 0..width {
            let column = grid.column(x);
            let start = column.iter().position(|t| *t != Tile::Void).unwrap();
            let end = height - column.iter().rev().position(|t| *t != Tile::Void).unwrap() - 1;
            col_limits.push((start, end));
        }

        let mut row_limits = Vec::new();
        for y in 0..height {
            let row = grid.row(y);
            let start = row.iter().position(|t| *t != Tile::Void).unwrap();
            let end = width - row.iter().rev().position(|t| *t != Tile::Void).unwrap() - 1;
            row_limits.push((start, end));
        }

        return Map { grid, row_limits, col_limits };

    }
}

impl MapState {

    fn new(input: &str) -> MapState {

        let mut lines:Vec<&str> = input.lines().collect();
        let instruction_line = lines.pop().unwrap();
        lines.pop(); // empty line

        let map = Map::new(lines);
        let instructions = MapState::parse_instructions(instruction_line);
        let x = map.row_limits[0].0;

        return MapState { map, instructions, x, y: 0, facing: 0 }

    }

    fn parse_instructions(input: &str) -> Vec<String> {
        let mut instructions:Vec<String> = Vec::new();
        let mut num = String::new();
        for c in input.chars() {
            if c == 'L' || c == 'R' {
                if num.len() > 0 {
                    instructions.push(num);
                    num = String::new();
                }
                instructions.push(c.to_string());
            } else {
                num.push(c);
            }            
        }
        if num.len() > 0 {
            instructions.push(num);
        }
        return instructions;
    }

    fn navigate(&mut self) {

        for i in self.instructions.clone() {
            if i == "L" {
                self.facing = (self.facing + 3) % 4;
            } else if i == "R" {
                self.facing = (self.facing + 1) % 4;
            } else {
                let dist = i.parse::<usize>().unwrap();
                for _ in 0..dist {
                    if !self.step_forward() {
                        break;
                    } 
                }
            }
        }
    }

    fn step_forward(&mut self) -> bool {

        let mut new_x = self.x;
        let mut new_y = self.y;
        let limits = if self.facing % 2 == 0 { self.map.row_limits[self.y] } else { self.map.col_limits[self.x] };

        if self.facing == 0 { // facing right
            new_x = if self.x == limits.1 { limits.0 } else { self.x + 1 };
        } else if self.facing == 1 { // facing down
            new_y = if self.y == limits.1 { limits.0 } else { self.y + 1 };
        } else if self.facing == 2 { // facing left
            new_x = if self.x == limits.0 { limits.1 } else { self.x - 1 };
        } else if self.facing == 3 { // facing up
            new_y = if self.y == limits.0 { limits.1 } else { self.y - 1 };
        }

        if self.map.grid[(new_y, new_x)] == Tile::Path {
            self.x = new_x;
            self.y = new_y;
            return true;
        } else {
            return false;
        }
    }

    fn get_password(&self) -> usize {
        return (1000 * (self.y + 1)) + (4 * (self.x + 1)) + self.facing;
    }

}