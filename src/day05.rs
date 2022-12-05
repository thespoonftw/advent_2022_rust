use crate::problem::Problem;

pub struct Day05();

impl Problem for Day05 {

    fn part_one(&self, input: &str) -> String {
        let mut s = StackSolver::init(input);
        s.move_boxes_pt1();
        return s.get_result();
    }

    fn part_two(&self, input: &str) -> String {
        let mut s = StackSolver::init(input);
        s.move_boxes_pt2();
        return s.get_result();
    }
}

pub struct StackSolver {
    stacks: Vec<Vec<char>>,
    instructions: Vec<Vec<usize>>
}

impl StackSolver {

    fn init(input: &str) -> StackSolver {

        let lines:Vec<&str> = input.lines().map(|l| l).collect();
        let width = (lines.get(0).unwrap().len() + 1) / 4;
        let height = lines.iter().position(|l| l.len() == 0).unwrap() - 1;
        let mut stacks: Vec<Vec<char>> = Vec::new();

        for i in 0..width {

            let mut s:Vec<char> = Vec::new();
            for j in (0..height).rev() {

                let x = (i * 4) + 1;
                let c = lines.get(j).unwrap().chars().nth(x).unwrap();
                if c == ' ' {
                    break;
                }
                s.push(c);
            }
            stacks.push(s);
        }

        let lines2:Vec<&str> = lines.iter().skip(height + 2).map(|l| l.clone()).collect();
        let instructions: Vec<Vec<usize>> = lines2.iter().map(|l| line_to_instruction(l.clone())).collect();

        return StackSolver{stacks, instructions}

    }

    fn move_boxes_pt1(&mut self) {
        for ins in &self.instructions.clone() {
            let count = ins.get(0).unwrap().clone();
            for _ in 0..count {
                let c = self.pop(ins.get(1).unwrap());
                self.push(ins.get(2).unwrap(), c);
            }
        }
    }

    fn move_boxes_pt2(&mut self) {
        for ins in &self.instructions.clone() {
            let count = ins.get(0).unwrap().clone();
            let mut stack:Vec<char> = Vec::new();
            for _ in 0..count {
                let c = self.pop(ins.get(1).unwrap());
                stack.push(c);
            }
            for _ in 0..count {
                let c = stack.pop().unwrap();
                self.push(ins.get(2).unwrap(), c);
            }
        }
    }

    fn pop(&mut self, stack_number: &usize) -> char {
        return self.stacks.get_mut(stack_number - 1).unwrap().pop().unwrap();
    }

    fn push(&mut self, stack_number: &usize, c: char) {
        return self.stacks.get_mut(stack_number - 1).unwrap().push(c);
    }

    fn get_result(&self) -> String {
        let mut returner = String::new();
        for t in &self.stacks {
            returner.push(t.iter().last().unwrap().clone());
        }
        return returner;
    }

}

// converts a instruction string to a 3 integer Vec.
fn line_to_instruction(line: &str) -> Vec<usize> {
    let split:Vec<&str> = line.split(' ').collect();
    return split.iter().skip(1).step_by(2).map(|s| s.clone().parse().unwrap()).collect();
}