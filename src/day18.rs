use crate::problem::Problem;
use ndarray::Array3;

pub struct Day18();

impl Problem for Day18 {

    fn part_one(&self, input: &str) -> String {

        let obsidian = Obsidian::new(input);
        return obsidian.find_surface_area().to_string();
    }

    fn part_two(&self, input: &str) -> String {
        
        let mut obsidian = Obsidian::new(input);
        obsidian.remove_air_pockets();
        return obsidian.find_surface_area().to_string();
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Material {
    Rock,
    AirPocket,
    Air,
}

struct Obsidian {
    array: Array3<Material>,
    size: usize
}

impl Obsidian {

    fn new(input: &str) -> Obsidian {

        let size = 50;
        let mut array:Array3<Material> = Array3::from_elem((size, size, size), Material::Air); 

        for line in input.lines() {
            let split:Vec<usize> = line.split(',').into_iter().map(|s| s.parse().unwrap()).collect();
            let x = split.get(0).unwrap().clone();
            let y = split.get(1).unwrap().clone();
            let z = split.get(2).unwrap().clone();
            array[(x, y, z)] = Material::Rock;
        }

        return Obsidian { size, array }
    }

    fn find_surface_area(&self) -> usize {
        let mut counter = 0;

        for x in 0..self.size {
            for y in 0..self.size {
                for z in 0..self.size {
                    counter += self.count_faces(x, y, z);                 
                }
            }
        }

        return counter;
    }

    fn remove_air_pockets(&mut self) {

        // replace all air with air pocket
        for mat in self.array.iter_mut() {
            if *mat == Material::Air {
                *mat = Material::AirPocket;
            }
        }

        // starting from 0,0,0. Mark all touching air pockets as air
        let mut stack = Vec::<(usize, usize, usize)>::new();
        stack.push((0,0,0));
        let max = self.size - 1;

        while !stack.is_empty() {

            let pos = stack.pop().unwrap();
            let mat = self.array[pos];
            let x = pos.0;
            let y = pos.1;
            let z = pos.2;

            if mat != Material::AirPocket { continue; }
            self.array[pos] = Material::Air;

            if x > 0 { stack.push((x-1, y, z)); }
            if y > 0 { stack.push((x, y-1, z)); }
            if z > 0 { stack.push((x, y, z-1)); }
            if x < max { stack.push((x+1, y, z)); }
            if y < max { stack.push((x, y+1, z)); }
            if z < max { stack.push((x, y, z+1)); }
        }
    }

    fn count_faces(&self, x: usize, y: usize, z: usize) -> usize {

        if self.array[(x,y,z)] != Material::Rock { return 0; }

        let air = Material::Air;

        let mut counter = 0;
        if self.array[(x+1,y,z)] == air { counter += 1; }
        if x==0 || self.array[(x-1,y,z)] == air { counter += 1; }
        if self.array[(x,y+1,z)] == air { counter += 1; }
        if y==0 || self.array[(x,y-1,z)] == air { counter += 1; }
        if self.array[(x,y,z+1)] == air { counter += 1; }
        if z==0 || self.array[(x,y,z-1)] == air { counter += 1; }
        return counter;
    }
}