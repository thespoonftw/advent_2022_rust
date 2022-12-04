use crate::problem::Problem;

pub struct Day01();

impl Problem for Day01 {

    fn part_one(&self, input: &str) -> String {
        return get_calories(input).iter().max().unwrap().to_string();
    }

    fn part_two(&self, input: &str) -> String {
        let mut calories = get_calories(input);
        calories.sort_by(|a, b| b.cmp(a));
        let sum = calories[0] + calories[1] + calories[2];
        return sum.to_string();
    }
}

fn get_calories(input: &str) -> Vec<i32> {
        
    let mut current = 0;
    let mut calories: Vec<i32> = Vec::new();

    for line in input.lines() {
        if line.len() == 0 {
            calories.push(current);
            current = 0;

        } else {
            let value = line.parse::<i32>().expect("Failed to parse");
            current += value;
        }
    }
    return calories;

}