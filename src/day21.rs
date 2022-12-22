use std::collections::HashMap;

use crate::problem::Problem;

pub struct Day21();

impl Problem for Day21 {

    fn part_one(&self, input: &str) -> String {

        let monkeys = Monkeys::new(input);
        return monkeys.evaluate_monkey_recurse("root".to_string()).unwrap().to_string();
    }

    fn part_two(&self, input: &str) -> String {
        let mut monkeys = Monkeys::new(input);
        monkeys.hashmap.remove("humn");

        let root = monkeys.hashmap[&"root".to_string()].clone();    
        let lhs = monkeys.evaluate_monkey_recurse(root.clone().lhs.unwrap());
        let rhs = monkeys.evaluate_monkey_recurse(root.clone().rhs.unwrap());
        let monkey_to_start = if lhs.is_none() { root.lhs.unwrap() } else { root.rhs.unwrap() };
        let value_to_match = if lhs.is_none() { rhs.unwrap() } else { lhs.unwrap() };
    
        return monkeys.evaluate_humn_recurse(monkey_to_start, value_to_match).to_string();
    }
}

#[derive(Clone, Debug)]
struct Monkey {
    name: String,
    value: Option<i64>,
    lhs: Option<String>,
    rhs: Option<String>,
    operation: Option<char>, 
}

impl Monkey {
    fn new(input: &str) -> Monkey {
        let split:Vec<&str> = input.split_whitespace().collect();
        let name = (split[0])[0..4].to_string();
    
        if split.len() == 2 {
            let v = split[1].parse::<i64>().unwrap();
            return Monkey { name, value: Some(v), lhs: None, rhs: None, operation: None };
    
        } else {
            let lhs = Some(split[1].to_string());
            let operation = Some(split[2].chars().next().unwrap());
            let rhs = Some(split[3].to_string());
            return Monkey { name, value: None, lhs, rhs, operation };
        }
    }
}

struct Monkeys {
    hashmap: HashMap<String, Monkey>
}

impl Monkeys {

    fn new(input: &str) -> Monkeys {
        return Monkeys { hashmap: input.lines().map(|l| Monkey::new(l)).map(|m| (m.clone().name, m)).collect() };
    }

    fn evaluate_monkey_recurse(&self, name: String) -> Option<i64> {

        if !self.hashmap.contains_key(&name) { return None; }
        let monkey = self.hashmap[&name].clone();
    
        if monkey.value.is_some() { return Some(monkey.value.unwrap()); }
    
        let lhs = self.evaluate_monkey_recurse(monkey.lhs.unwrap());
        let rhs = self.evaluate_monkey_recurse(monkey.rhs.unwrap());
        if lhs.is_none() || rhs.is_none() { return None }
    
        return Some(operation(lhs.unwrap(), rhs.unwrap(), monkey.operation.unwrap()));    
    }
    
    fn evaluate_humn_recurse(&self, name: String, value_so_far: i64) -> i64 {
    
        if name == "humn" { return value_so_far; }
        let monkey = self.hashmap[&name].clone();
        let left_name = monkey.lhs.unwrap();
        let right_name = monkey.rhs.unwrap();
        let operation = monkey.operation.unwrap();
        let lhs = self.evaluate_monkey_recurse(left_name.clone());
        let rhs = self.evaluate_monkey_recurse(right_name.clone());        

        if lhs.is_none() {
            let a = find_left_op(rhs.unwrap(), value_so_far, operation);
            return self.evaluate_humn_recurse(left_name, a);
        }
        else {
            let b = find_right_op(lhs.unwrap(), value_so_far, operation);
            return self.evaluate_humn_recurse(right_name, b);
        }
    }
    
}

fn operation(a: i64, b: i64, operation: char) -> i64 {
    match operation {
        '+' => return a + b,
        '-' => return a - b,
        '*' => return a * b,
        '/' => return a / b,
        _ => return 0,
    }
}

fn find_right_op(a: i64, c: i64, operation: char) -> i64 {
    match operation {
        '+' => return c - a,
        '-' => return a - c,
        '*' => return c / a,
        '/' => return a / c,
        _ => return 0,
    }
}

fn find_left_op(b: i64, c: i64, operation: char) -> i64 {
    match operation {
        '+' => return c - b,
        '-' => return b + c,
        '*' => return c / b,
        '/' => return b * c,
        _ => return 0,
    }
}