use crate::problem::Problem;

pub struct Day04();

impl Problem for Day04 {

    fn part_one(&self, input: &str) -> String {
        return input.lines().filter(|l| is_contained(l)).count().to_string();
    }

    fn part_two(&self, input: &str) -> String {
        return input.lines().filter(|l| is_overlap(l)).count().to_string();
    }
}

fn is_contained(line: &str) -> bool {
    let s = split_line(line);
    return (s[0] >= s[2] && s[1] <= s[3]) || (s[2] >= s[0] && s[3] <= s[1]);
}

fn is_overlap(line: &str) -> bool {
    let s = split_line(line);
    return !(s[0] > s[3] || s[1] < s[2]);
}

fn split_line(line: &str) -> Vec<i32> {
    return line.split([',', '-']).map(|c| c.parse().unwrap()).collect();
}