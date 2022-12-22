
use crate::problem::Problem;

pub struct Day20();

impl Problem for Day20 {

    fn part_one(&self, input: &str) -> String {
        let values = parse_values(input);
        let mixed_values = mix_values(values, 1);
        return find_coordinates(mixed_values).to_string();
    }

    fn part_two(&self, input: &str) -> String {
        let values = parse_values(input);
        let encrypted_values = values.iter().map(|v| v * 811589153).collect();
        let mixed_values = mix_values(encrypted_values, 10);
        return find_coordinates(mixed_values).to_string();
    }
}

fn parse_values(input: &str) -> Vec<i64> {
    return input.lines().into_iter().map(|l| l.parse().unwrap()).collect();
}

fn mix_values(values: Vec<i64>, iterations: usize) -> Vec<i64> {

    let len32 = values.len() as i64;
    let len = values.len();

    let mut circular_list:Vec<(i64, usize)> = Vec::new();
    for i in 0..len {
        circular_list.push((values[i], i));
    }

    for _ in 0..iterations {
        for i in 0..len {

            let result = circular_list.iter().enumerate().find(|(_, x)| x.1 == i).unwrap();
            let start_position = result.0;
            let value = (result.1).0;
            
            let end_position = (start_position as i64 + value).rem_euclid(len32 - 1) as usize;
    
            let tail = circular_list.split_off(start_position + 1);
            let popped = circular_list.pop().unwrap();
            circular_list.extend(tail);
    
            if end_position == 0 {
                circular_list.push(popped);
            } else {
                let tail2 = circular_list.split_off(end_position);
                circular_list.push(popped);
                circular_list.extend(tail2);
            }  
        }
    }
    
    return circular_list.clone().iter().map(|t| t.0.clone()).collect();;

}

fn find_coordinates(values: Vec<i64>) -> i64 {

    let len = values.len();
    let count_from = values.iter().position(|x| *x == 0).unwrap();
    
    let a = values[(count_from + 1000) % len];
    let b = values[(count_from + 2000) % len];
    let c = values[(count_from + 3000) % len];

    return a + b + c;
}