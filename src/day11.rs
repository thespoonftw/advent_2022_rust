use crate::problem::Problem;

pub struct Day11();

impl Problem for Day11 {

    fn part_one(&self, input: &str) -> String {
        return find_monkey_business(input, true).to_string();
    }

    fn part_two(&self, input: &str) -> String {
        return find_monkey_business(input, false).to_string();
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Operation {
    Add,
    Multiply,
    Power,
}

struct Monkey {
    no_of_inspects: usize,
    held_items: Vec<usize>,
    op: Operation,
    op_value: usize,
    divisor: usize,
    true_target: usize,
    false_target: usize,
}

impl Monkey {

    fn new(input: &[&str]) -> Monkey {

        let items_line = input[1];
        let mut held_items: Vec<usize> = items_line[18..].split(", ").into_iter().map(|s| s.parse::<usize>().unwrap()).collect();
        held_items.reverse();

        let operation_line = input[2].split_whitespace().collect::<Vec<&str>>();
        let mut op = Operation::Add;
        let mut op_value = 2;
        if operation_line[5] == "old" {
            op = Operation::Power;
        } else {
            op_value = operation_line[5].parse().unwrap();
            if operation_line[4] == "*" {
                op = Operation::Multiply;
            }
        }

        let divisor = input[3].split_whitespace().nth(3).unwrap().parse().unwrap();
        let true_target = input[4].split_whitespace().nth(5).unwrap().parse().unwrap();
        let false_target = input[5].split_whitespace().nth(5).unwrap().parse().unwrap();

        return Monkey { no_of_inspects: 0, held_items, op, op_value, divisor, true_target, false_target }
    }

}


fn find_monkey_business(input: &str, is_part_1: bool) -> usize {

    let mut monkeys = input.lines()
        .collect::<Vec<&str>>()
        .chunks(7)
        .map(|c| Monkey::new(c))
        .collect::<Vec<Monkey>>();

    let max_size = monkeys.iter().map(|m| m.divisor).product::<usize>();
    let rounds = if is_part_1 { 20 } else { 10000 };
    let divisor = if is_part_1 { 3 } else { 1 };

    for _ in 0..rounds {
        run_cycle(&mut monkeys, divisor, max_size);
    }

    monkeys.sort_by(|m1, m2| m1.no_of_inspects.cmp(&m2.no_of_inspects));
    let l = monkeys.len();
    let m1 = monkeys.get(l - 1).unwrap().no_of_inspects;
    let m2 = monkeys.get(l - 2).unwrap().no_of_inspects;
    return m1 * m2;
}

fn run_cycle(monkeys: &mut Vec<Monkey>, divisor: usize, max_size: usize) {

    for i in 0..monkeys.len() {
        for _ in 0..monkeys.get(i).unwrap().held_items.len() {
            let m = monkeys.get_mut(i).unwrap();
            let item = m.held_items.pop().unwrap();
            m.no_of_inspects += 1;

            let mut new_value = apply_operation(item, m.op, m.op_value);
            new_value /= divisor;
            new_value %= max_size;
            let target = if new_value % m.divisor == 0 { m.true_target } else { m.false_target };

            let m2 = monkeys.get_mut(target).unwrap();
            m2.held_items.insert(0, new_value);
        }
    }
}

fn apply_operation(input: usize, op: Operation, op_value: usize) -> usize {
    if op == Operation::Add {
        return input + op_value;
    } else if op == Operation::Multiply {
        return input * op_value;
    } else if op == Operation::Power {
        return input * input;
    } else {
        return 0;
    }
}