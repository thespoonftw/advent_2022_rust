use crate::problem::Problem;
use ndarray::Array2;

pub struct Day22();

impl Problem for Day22 {

    fn part_one(&self, input: &str) -> String {
        let mut map = Map::new(input, true);        
        map.navigate();
        return map.get_password().to_string();
    }

    fn part_two(&self, input: &str) -> String {
        let mut map = Map::new(input, false);        
        map.navigate();
        return map.get_password().to_string();
    }
}

struct Map {
    grid: Vec<Array2<bool>>,
    instructions: Vec<String>,
    loc: Location,
    face_offsets: Vec<(usize, usize)>,
    size: usize,
    connections: Vec<Vec<(usize, usize)>>,
}

#[derive(Clone, Copy, Debug)]
struct Location {
    x: usize,
    y: usize,
    dir: usize,
    face: usize,
}

impl Map {

    fn new(input: &str, is_flat: bool) -> Map {

        let mut lines:Vec<&str> = input.lines().collect();

        let instruction_line = lines.pop().unwrap();
        lines.pop(); // remove empty line
        let instructions = parse_instructions(instruction_line);

        let is_test = lines.len() < 20;
        let loc = Location { x: 0, y: 0, dir: 0, face: 0 };
        let size = get_size(is_test);
        let face_offsets = get_face_offset(is_test);
        let connections = get_connections(is_test, is_flat);
        
        let mut grid = Vec::new();
        let chars:Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
        for f in 0..6 {
            let mut face = Array2::from_elem((size, size), false);
            let offset = face_offsets[f];
            for x in 0..size {
                for y in 0..size {
                    face[(y,x)] = chars[y+offset.1][x+offset.0] == '#';           
                }
            }
            grid.push(face);
        }

        return Map { grid, instructions, loc, size, face_offsets, connections }
    }

    fn navigate(&mut self) {

        for i in self.instructions.clone() {
            if i == "L" {
                self.loc.dir = (self.loc.dir + 3) % 4;
            } else if i == "R" {
                self.loc.dir = (self.loc.dir + 1) % 4;
            } else {
                let dist = i.parse::<usize>().unwrap();
                for _ in 0..dist {
                    let next = self.get_next_position();
                    let is_wall = self.grid[next.face][(next.y, next.x)];
                    if is_wall {
                        break;
                    } else {
                        self.loc = next;
                    } 
                }
            }
        }
    }

    fn get_next_position(&self) -> Location {

        let s = self.size - 1;
        let mut loc = self.loc.clone();
        let connection = self.connections[loc.face][loc.dir];

        if loc.dir == 0 {
            if loc.x == s { // right edge                
                return self.get_face_location(loc.y, connection.1, connection.0);
            } else {
                loc.x += 1;
            }
        } else if loc.dir == 1 {
            if loc.y == s { // bottom edge
                return self.get_face_location(s - loc.x, connection.1, connection.0);
            } else {
                loc.y += 1;
            }
        } else if loc.dir == 2 {
            if loc.x == 0 { // left edge
                return self.get_face_location(s - loc.y, connection.1, connection.0);
            } else {
                loc.x -= 1;
            }
        } else if loc.dir == 3 {
            if loc.y == 0 { // top edge
                return self.get_face_location(loc.x, connection.1, connection.0);
            } else {
                loc.y -= 1;
            }
        }
        return loc;
    }

    fn get_face_location(&self, edge_position: usize, dir: usize, face: usize) -> Location {
        let xy = self.get_face_position(edge_position, dir);
        return Location { x: xy.0, y: xy.1, dir, face }
    }

    fn get_face_position(&self, edge_position: usize, dir: usize) -> (usize, usize) { // (x, y)
        let s = self.size - 1;
        match dir {
            0 => return ( 0, edge_position ),
            1 => return ( s - edge_position, 0 ),
            2 => return ( s, s - edge_position ),
            3 => return ( edge_position, s ),
            _ => return ( 0, 0 ),
        }
    }

    fn get_password(&self) -> usize {
        let offset = self.face_offsets[self.loc.face];
        return (1000 * (self.loc.y + offset.1 + 1)) + (4 * (self.loc.x + offset.0 + 1)) + self.loc.dir;
    }

}

fn parse_instructions(instruction_line: &str) -> Vec<String> {
    let mut instructions:Vec<String> = Vec::new();
    let mut num = String::new();
    for c in instruction_line.chars() {
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

#[allow(dead_code)]
fn print_grid(grid: &Array2<bool>) {
    for row in grid.rows() {        
        for b in row {
            let c = if *b { '#' } else { '.' };
            print!("{c}");
        }
        println!();
    }
    println!();
}

fn get_size(is_test: bool) -> usize {
    return if is_test { 4 } else { 50 };
}

fn get_face_offset(is_test: bool) -> Vec<(usize, usize)> {
    if is_test {
        return vec![(8, 0), (0, 4), (4, 4), (8, 4), (8, 8), (12, 8)];
    } else {
        return vec![(50, 0), (100, 0), (50, 50), (0, 100), (50, 100), (0, 150)];
    }
}

fn get_connections(is_test: bool, is_flat: bool) -> Vec<Vec<(usize, usize)>> { // (face, direction)
    if is_test && is_flat {
        return vec![
            vec![(0, 0), (3, 1), (0, 2), (4, 3)], // face 0
            vec![(2, 0), (1, 1), (3, 2), (1, 3)], // face 1
            vec![(3, 0), (2, 1), (1, 2), (2, 3)], // face 2
            vec![(1, 0), (4, 1), (2, 2), (0, 3)], // face 3
            vec![(5, 0), (0, 1), (5, 2), (3, 3)], // face 4
            vec![(4, 0), (5, 1), (4, 2), (5, 3)], // face 5
        ];
    } else if !is_test && is_flat {
        return vec! [
            vec![(1, 0), (2, 1), (1, 2), (4, 3)], // face 0
            vec![(0, 0), (1, 1), (0, 2), (1, 3)], // face 1
            vec![(2, 0), (4, 1), (2, 2), (0, 3)], // face 2
            vec![(4, 0), (5, 1), (4, 2), (5, 3)], // face 3
            vec![(3, 0), (0, 1), (3, 2), (2, 3)], // face 4
            vec![(5, 0), (3, 1), (5, 2), (3, 3)], // face 5
        ];
    } else if is_test && !is_flat {
        return vec! [
            vec![(5, 0), (3, 1), (2, 1), (1, 1)], // face 0
            vec![(2, 0), (4, 3), (5, 3), (0, 1)], // face 1
            vec![(3, 0), (4, 0), (1, 2), (0, 0)], // face 2
            vec![(5, 1), (4, 1), (2, 2), (0, 3)], // face 3
            vec![(5, 0), (1, 3), (2, 3), (3, 3)], // face 4
            vec![(0, 2), (1, 0), (4, 2), (3, 2)], // face 5
        ];
    } else {
        return vec! [
            vec![(1, 0), (2, 1), (3, 0), (5, 0)], // face 0
            vec![(4, 2), (2, 2), (0, 2), (5, 3)], // face 1
            vec![(1, 3), (4, 1), (3, 1), (0, 3)], // face 2
            vec![(4, 0), (5, 1), (0, 0), (2, 0)], // face 3
            vec![(1, 2), (5, 2), (3, 2), (2, 3)], // face 4
            vec![(4, 3), (1, 1), (0, 1), (3, 3)], // face 5
        ];
    }
}