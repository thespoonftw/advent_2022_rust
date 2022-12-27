use std::collections::HashSet;

use crate::problem::Problem;
use ndarray::Array2;

pub struct Day24();

impl Problem for Day24 {

    fn part_one(&self, input: &str) -> String {
        let b = Blizzard::new(input);
        return b.search(true, 0).to_string();
    }

    fn part_two(&self, input: &str) -> String {
        let b = Blizzard::new(input);
        let t1 = b.search(true, 0);
        let t2 = b.search(false, t1);
        let t3 = b.search(true, t2);
        return t3.to_string();
    }
}

struct Blizzard {
    width: usize,
    height: usize,
    tiles: Array2<Tile>,
}

#[derive(Clone, Debug)]
struct Tile {
    h_check: Vec<bool>,
    v_check: Vec<bool>,
}

impl Blizzard {

    fn new(input: &str) -> Blizzard {

        let mut lines:Vec<&str> = input.lines().collect();
        lines.drain(0..1); // remove first
        lines.pop(); // remove last
        lines = lines.iter().map(|s| s.get(1..(s.len()-1)).unwrap()).collect(); // remove first and last chars
        let height = lines.len();
        let width = lines[0].len();
        let char_vec:Vec<Vec<char>> = lines.into_iter().map(|s| s.chars().collect()).collect();
        let flatten:Vec<char> = char_vec.into_iter().flatten().collect();
        let chars:Array2<char> = Array2::from_shape_vec((height, width), flatten).unwrap();

        let mut tile_vec:Vec<Tile> = Vec::new();
        for y in 0..height {
            for x in 0..width {
                tile_vec.push(Blizzard::make_tile(&chars, x, y, width, height));
            }
        }
        let tiles = Array2::from_shape_vec((height, width), tile_vec).unwrap();

        return Blizzard { width, height, tiles }

    }

    fn search(&self, is_forward: bool, mut t: usize) -> usize {
        let width = self.width - 1;
        let height = self.height - 1;
        let start = if is_forward { (0, 0) } else { (width, height) };
        let end   = if is_forward { (width, height) } else { (0, 0) };
        let mut positions: HashSet<(usize, usize)> = HashSet::new();

        loop {
            t += 1;
            let mut new_set: HashSet<(usize, usize)> = HashSet::new();
            self.add_if_clear(start.0, start.1, t, &mut new_set);

            for (x, y) in positions.into_iter() {
                if x == end.0 && y == end.1 { return t; }
                if x > 0 { self.add_if_clear(x - 1, y, t, &mut new_set); }
                if y > 0 { self.add_if_clear(x, y - 1, t, &mut new_set); }
                self.add_if_clear(x, y, t, &mut new_set);
                if y < height { self.add_if_clear(x, y + 1, t, &mut new_set); }
                if x < width { self.add_if_clear(x + 1, y, t, &mut new_set); }
            }
            positions = new_set;
        }
    }

    fn add_if_clear(&self, x: usize, y: usize, t: usize, set: &mut HashSet<(usize, usize)>) {
        let tile = self.tiles[(y, x)].clone();
        let v_t = t % self.height;
        let h_t = t % self.width;
        let b1 = tile.v_check[v_t];
        let b2 = tile.h_check[h_t];
        if !b1 || !b2 { return; }
        set.insert((x, y));
    }

    fn make_tile(chars: &Array2<char>, x: usize, y: usize, width: usize, height: usize) -> Tile {
        let mut h_check = Vec::new();
        for i in 0..width {
            let x1 = (x + i) % width;
            let x2 = (x as i32 - i as i32).rem_euclid(width as i32) as usize;
            let b1 = chars[(y, x1)] != '<';
            let b2 = chars[(y, x2)] != '>';
            h_check.push(b1 && b2);
        }
        let mut v_check = Vec::new();
        for i in 0..height {
            let y1 = (y + i) % height;
            let y2 = (y as i32 - i as i32).rem_euclid(height as i32) as usize;
            let b1 = chars[(y1, x)] != '^';
            let b2 = chars[(y2, x)] != 'v';
            v_check.push(b1 && b2);
        }
        return Tile { h_check, v_check };
    }
    
}