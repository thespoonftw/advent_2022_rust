use crate::problem::Problem;

pub struct Day07();

impl Problem for Day07 {

    fn part_one(&self, input: &str) -> String {
        let v = parse(input);
        return sum_totals(v, 100000).to_string();
    }

    fn part_two(&self, input: &str) -> String {
        let v = parse(input);
        return file_to_delete(v, 70000000, 30000000).to_string();
    }
}

pub struct DirectoryParser {

    stack: Vec<usize>,
    returner: Vec<usize>,
    current_sum: usize,
}

impl DirectoryParser {
    
    fn new() -> DirectoryParser {
        return DirectoryParser {
            stack: Vec::<usize>::new(),
            returner: Vec::<usize>::new(),
            current_sum: 0
        }
    }

    fn go_up(&mut self) {
        let inner_sum = self.current_sum.clone();
        self.returner.push(self.current_sum);
        self.current_sum = self.stack.pop().unwrap() + inner_sum;
    }

    fn go_down(&mut self) {
        self.stack.push(self.current_sum);
        self.current_sum = 0;
    }

    fn add_file(&mut self, file_size: usize) {
        self.current_sum += file_size;
    }

    fn empty_stack(&mut self) {
        while self.stack.len() > 0 {
            self.go_up();
        }
    }
}

fn parse(input: &str) -> Vec<usize> {
    let mut parser = DirectoryParser::new();

    for line in input.lines() {
        if line.starts_with("$ cd ..") {
            parser.go_up();
        }
        else if line.starts_with("$ cd") {
            parser.go_down();            
        }
        else if !line.starts_with("$ ls") && !line.starts_with("dir") {
            parser.add_file(line.split(' ').next().unwrap().parse::<usize>().unwrap());
        }
    }
    parser.empty_stack();

    return parser.returner;
}

fn sum_totals(v: Vec<usize>, threshold: usize) -> usize {
    return v.iter().filter(|i| i <= &&threshold).sum();
}

fn file_to_delete(v: Vec<usize>, total_disk_space: usize, required_space: usize) -> usize {
    let used_space = v.last().unwrap().clone();
    let unused_space = total_disk_space - used_space;
    let delete_size = required_space - unused_space;
    return v.iter().filter(|i| i >= &&delete_size ).min().unwrap().clone();
}