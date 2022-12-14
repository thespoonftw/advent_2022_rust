use crate::problem::Problem;
//use ndarray::Array2;

pub struct Day12();

impl Problem for Day12 {

    fn part_one(&self, input: &str) -> String {
        //let s = Searcher::new(input);
        return input.len().to_string();
    }

    fn part_two(&self, input: &str) -> String {
        return input.len().to_string();
    }
}

/* SOLVED ON PAPER IN THE END


struct Node {
    height: char,
    x: usize,
    y: usize,
    prev_x: usize,
    prev_y: usize,
    g_score: usize,
    f_score: usize,
}

struct Searcher {
    map: Array2<Node>,
    width: usize,
    height: usize,
    max_dist: usize,
    visited: Vec<Node>,
    unvisited: Vec<Node>,
}

impl Searcher {

    fn new(input: &str) -> Searcher {

        let height = input.lines().count();
        let width = input.lines().into_iter().nth(0).unwrap().len();
        let max_dist = height + width;

        let visited = Vec::<Node>::new();
        let unvisited = Vec::<Node>::new();

        let mut nodes = Vec::<Node>::new();
        //let min_height = 'a' as usize;

        let mut y = 0;
        for line in input.lines() {
            let mut x = 0;
            for char in line.chars() {
                nodes.push(Node{
                    height: char,
                    x,
                    y,
                    prev_x: max_dist,
                    prev_y: max_dist,
                    g_score: max_dist,
                    f_score: max_dist,
                });
                x += 1;
            }
            y += 1;
        }

        let map: Array2<Node> = Array2::from_shape_vec((width, height), nodes).unwrap();

        return Searcher { map, width, height, max_dist, visited, unvisited }
    }


}
*/