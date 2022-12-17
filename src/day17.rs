use crate::problem::Problem;
use ndarray::Array2;

pub struct Day17();

impl Problem for Day17 {

    fn part_one(&self, input: &str) -> String {
        let mut sim = TetrisSim::new(input);

        for _ in 0..2022 {
            sim.simulate_rock(); 
        }

        return sim.height.to_string();
    }

    fn part_two(&self, input: &str) -> String {

        // solved on paper, sorry folks

        return input.len().to_string();
    }
}

struct TetrisSim {
    jet_pattern: Vec<bool>,
    jet_index: usize,
    rock_index: usize,
    grid: Array2<bool>,
    height: usize,
}

impl TetrisSim {

    fn new(input: &str) -> TetrisSim {

        let jet_pattern = input.chars().into_iter().map(|c| c == '>').collect();
        let width = 7;
        let max_height = 100_000_000;
        let grid:Array2<bool> = Array2::from_elem((width, max_height), false); 
        return TetrisSim { jet_pattern, jet_index: 0, rock_index: 0, grid, height: 0 }

    }

    fn simulate_rock(&mut self) {

        let mut rock_y: usize = self.height + 3; // 3 above highest rock
        let mut rock_x: i32 = 2; // 2 from left edge
        let rock: Array2<bool> = self.get_next_rock();
        
        loop {

            let jet_mod = self.get_next_jet_mod();

            if self.can_move(&rock, rock_x + jet_mod, rock_y) {
                rock_x += jet_mod;
            }

            if rock_y == 0 { break; }
            
            if self.can_move(&rock, rock_x, rock_y - 1) {
                rock_y -= 1;
            } else {
                break;
            }
        }
        
        self.place_rock(&rock, rock_x as usize, rock_y);

    }

    fn can_move(&mut self, rock: &Array2<bool>, rock_x: i32, rock_y: usize) -> bool {
        
        // boundaries
        if rock_x < 0 { return false; }
        if rock_x + rock.nrows() as i32 >= 8 { return false; }

        let r_x = rock_x as usize;
        let r_y = rock_y as usize;

        for x in 0..rock.nrows() {
            for y in 0..rock.ncols() {
                if rock[[x, y]] && self.grid[[r_x + x, r_y + y]] {
                    return false;
                }                
            }
        }

        return true;
    }


    fn get_next_jet_mod(&mut self) -> i32 {

        let is_jet_right = self.jet_pattern.get(self.jet_index).unwrap().clone();
        self.jet_index += 1;
        self.jet_index %= self.jet_pattern.len();
        return if is_jet_right { 1 } else { -1};
    }

    fn get_next_rock(&mut self) -> Array2<bool> {

        let i = self.rock_index.clone();
        self.rock_index += 1;
        self.rock_index %= 5;

        // horizontal line
        if i == 0 {
            return Array2::from_elem((4, 1), true); 

        // plus
        } else if i == 1 {
            let vec = vec![false, true, false, true, true, true, false, true, false];
            return Array2::from_shape_vec((3, 3), vec).unwrap();

        // reverse L
        } else if i == 2 {
            let vec = vec![true, false, false, true, false, false, true, true, true];
            return Array2::from_shape_vec((3, 3), vec).unwrap();

        // vertical line
        } else if i == 3 {
            return Array2::from_elem((1, 4), true); 

        // square
        } else {
            return Array2::from_elem((2, 2), true); 
        }
    }

    fn place_rock(&mut self, rock: &Array2<bool>, rock_x: usize, rock_y: usize) {

        for x in 0..rock.nrows() {
            for y in 0..rock.ncols() {
                if rock[[x, y]] {
                    self.grid[[rock_x + x, rock_y + y]] = true;
                }                
            }
        }

        let top = rock_y as usize + rock.ncols();
        if top > self.height { self.height = top; }
    }
    

}