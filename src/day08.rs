use crate::problem::Problem;

pub struct Day08();

impl Problem for Day08 {

    fn part_one(&self, input: &str) -> String {
        let mut grid = TreeGrid::new(input);
        return grid.count_visible_trees().to_string();
    }

    fn part_two(&self, input: &str) -> String {
        let mut grid = TreeGrid::new(input);
        return grid.max_visible_tree().to_string();
    }
}

struct TreeGrid {
    width: usize,
    heights: Vec<i32>,
    visibilities: Vec<usize>,
}

impl TreeGrid {

    // fill structure
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

    fn count_visible_trees(&mut self) -> usize {

        for line in self.get_lines() {
            self.mark_trees_visible(&line)
        }
        return self.visibilities.iter().sum();
    }

    fn max_visible_tree(&mut self) -> usize {

        self.visibilities = vec![1; self.width * self.width];
        for line in self.get_lines() {
            self.mark_trees_seen(&line)
        }
        return self.visibilities.iter().max().unwrap().clone();
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        return y * self.width + x;
    }

    // get every possible horizontal and vertical line through the grid
    fn get_lines(&self) -> Vec<Vec<usize>> {

        let mut returner = Vec::new();
        for i in 0..self.width {
            let line1 = (0..self.width).into_iter().map(|x| self.get_index(x, i)).collect::<Vec<usize>>();
            let line2 = (0..self.width).into_iter().map(|y| self.get_index(i, y)).collect::<Vec<usize>>();
            returner.push(line1.clone().into_iter().rev().collect::<Vec<usize>>());
            returner.push(line2.clone().into_iter().rev().collect::<Vec<usize>>());
            returner.push(line1);
            returner.push(line2);
        }
        return returner;
    }

    // go down the line, marking trees that we can see
    fn mark_trees_visible(&mut self, indexes: &Vec<usize>) {
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

    // go down the line, counting how many trees each tree can see
    fn mark_trees_seen(&mut self, indexes: &Vec<usize>) {
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
            self.visibilities[index] *= check_length; // multiply tree's visibility by how far it can see
        }
    }

}