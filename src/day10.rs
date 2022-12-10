use crate::problem::Problem;

pub struct Day10();

impl Problem for Day10 {

    fn part_one(&self, input: &str) -> String {
        let x_values = get_x_values(input);
        let arr = [20, 60, 100, 140, 180, 220];
        return arr.map(|a| (x_values[a] as usize) * a).iter().sum::<usize>().to_string();
    }

    fn part_two(&self, input: &str) -> String {
        let x_values = get_x_values(input);
        let r = 1..240;
        return r.into_iter().map(|i| find_char(i-1, x_values[i as usize])).collect();
    }
}

fn get_x_values(input: &str) -> Vec<i32> {
    let mut returner = Vec::<i32>::new();
    let mut x_value = 1;
    returner.push(1);
    returner.push(1);
    for line in input.lines() {
        returner.push(x_value);
        if line != "noop" {
            x_value += line.split_whitespace().collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
            returner.push(x_value);
        }
    }
    return returner;
}

fn find_char(cycle_num: i32, x_value: i32) -> String {
    let mut returner = String::new();
    if cycle_num % 40 == 0 {
        returner += "\n";
    }
    let line_pos = cycle_num % 40;
    let is_on = (line_pos - x_value).abs() <= 1;
    if is_on {
        returner += "#";
    } else {
        returner += ".";
    }
    return returner;
}