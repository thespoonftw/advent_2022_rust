use crate::problem::Problem;

pub struct Day10();

impl Problem for Day10 {

    fn part_one(&self, input: &str) -> String {
        return run(input).to_string();
    }

    fn part_two(&self, input: &str) -> String {
        return input.len().to_string();
    }
}

fn run(input: &str) -> i32 {

    let mut cycle_num = 0;
    let mut x_value = 1;
    let mut returner = 0;
    let mut check_values = vec![20, 60, 100, 140, 180, 220];
    let mut returner_string = "".to_string();

    for line in input.lines() {

        let mut x_adder = 0;
        let mut cycle_adder = 1;

        if line != "noop" {
            x_adder = line.split_whitespace().collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
            cycle_adder = 2;
        }

        // part 2 stuff
        for _ in 0..cycle_adder {

            if is_within_range(cycle_num, x_value) {
                returner_string += "#";
            } else {
                returner_string += ".";
            }

            cycle_num += 1;

            if check_values.len() != 0 && cycle_num >= check_values[0] {
                returner += x_value * check_values[0];
                check_values.remove(0);
            }
        }

        x_value += x_adder;
    }

    println!("{returner_string}");

    return returner;

}

fn is_within_range(cycle_num: i32, x_value: i32) -> bool {
    let line_pos = cycle_num % 40;
    return (line_pos - x_value).abs() <= 1;
}