

use crate::problem::Problem;

pub struct Day08();

impl Problem for Day08 {

    fn part_one(&self, input: &str) -> String {

        let mut grid = TreeGrid::new(input);
        grid.find_visibilities(true);
        return grid.count_visible().to_string();
    }

    fn part_two(&self, input: &str) -> String {
        
        let mut grid = TreeGrid::new(input);
        grid.find_visibilities(false);
        return grid.max_visible().to_string();
    }
}

struct TreeGrid {
    width: usize,
    heights: Vec<i32>,
    visibilities: Vec<usize>,
}

impl TreeGrid {

    fn new(input: &str) -> TreeGrid {
        let width = input.lines().count();
        let visibilities = vec![0; width * width];
        let mut heights = Vec::<i32>::new();
        
        for line in input.lines() {
            for c in line.chars() {
                heights.push(c.to_digit(10).unwrap() as i32);
            }
        }
        return TreeGrid { width, heights, visibilities }
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        return y * self.width + x;
    }

    fn count_visible(&self) -> usize {
        return self.visibilities.iter().sum();
    }

    fn max_visible(&self) -> usize {
        return self.visibilities.iter().max().unwrap().clone();
    }

    fn find_visibilities(&mut self, part1: bool) {

        if !part1 {
            self.visibilities = vec![1; self.width * self.width];
        }

        for y in 0..self.width {
            let mut line:Vec<usize> = (0..self.width).into_iter().map(|x| self.get_index(x, y)).collect();
            self.find_visibilities_line(&line, part1);
            line.reverse();
            self.find_visibilities_line(&line, part1);
        }
    
        for x in 0..self.width {
            let mut line:Vec<usize> = (0..self.width).into_iter().map(|y| self.get_index(x, y)).collect();
            self.find_visibilities_line(&line, part1);
            line.reverse();
            self.find_visibilities_line(&line, part1);
        }    
    }

    fn find_visibilities_line(&mut self, indexes: &Vec<usize>, part1: bool) {
        if part1 {
            self.find_visibilities_line_p1(indexes);
        } else {
            self.find_visibilities_line_p2(indexes);
        }
    }

    fn find_visibilities_line_p1(&mut self, indexes: &Vec<usize>) {
        let mut current_height = -1;
        for index in indexes.clone() {
            let h = self.heights[index];
            if h > current_height {
                current_height = h;
                self.visibilities[index] = 1;
            }
            if h == 9 {
                break;
            }
        }
    }

    fn find_visibilities_line_p2(&mut self, indexes: &Vec<usize>) {
        let len = indexes.len();

        for i in 0..len {
            let index = indexes[i];
            let height_from = self.heights[index];
            let mut check_length = 0;

            loop {

                if i + check_length + 1 >= len {
                    break;
                }

                check_length += 1;
                let height_to = self.heights[indexes[i + check_length]];

                if height_to >= height_from {
                    break;
                }

                
            }
            
            self.visibilities[index] *= check_length;
            
        }
    }

}