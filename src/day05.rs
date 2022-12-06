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

struct StackSolver {
    stacks: Vec<Vec<char>>,
    instructions: Vec<Instruction>
}

#[derive(Clone)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize
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
        let instructions: Vec<Instruction> = lines2.iter().map(|l| Instruction::new(l.clone())).collect();

        return StackSolver{stacks, instructions}

    }

    fn move_boxes_pt1(&mut self) {
        for ins in &self.instructions.clone() {
            for _ in 0..ins.amount {
                let c = self.stacks.get_mut(ins.from).unwrap().pop().unwrap();
                self.stacks.get_mut(ins.to).unwrap().push(c);
            }
        }
    }
    
    fn move_boxes_pt2(&mut self) {
        for ins in &self.instructions.clone() {
            let from = self.stacks.get_mut(ins.from).unwrap();
            let l = from.len() - ins.amount;
            let mut tail = from.split_off(l);
            self.stacks.get_mut(ins.to).unwrap().append(&mut tail);
        }
    }

    fn get_result(&self) -> String {
        return self.stacks.iter().map(|s| s.last().unwrap()).collect();
    }

    

}

impl Instruction {

    fn new(line: &str) -> Instruction {
        let split:Vec<usize> = line.split(' ').skip(1).step_by(2).map(|s| s.parse().unwrap()).collect();
        let amount = *split.get(0).unwrap();
        let from = *split.get(1).unwrap() - 1;
        let to = *split.get(2).unwrap() - 1;
        return Instruction { amount, from, to }
    }
}
