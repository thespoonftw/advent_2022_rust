use std::collections::HashSet;

use crate::problem::Problem;

pub struct Day03();

impl Problem for Day03 {

    fn part_one(&self, input: &str) -> String {
        let sum:usize = input.lines().map(|l| get_backpack_item(l)).sum();
        return sum.to_string();
    }

    fn part_two(&self, input: &str) -> String {
        let lines:Vec<&str> = input.lines().map(|l| l).collect();
        let mut i = 0;
        let l = lines.len();
        let mut sum = 0;
        while i < l {
            sum += get_badge(lines[i], lines[i + 1], lines[i + 2]);
            i += 3;
        }
        return sum.to_string();
    }
}

fn get_backpack_item(line: &str) -> usize {
    let l = line.len() / 2;
    let front:Vec<char> = line.chars().take(l).collect();
    let back:Vec<char> = line.chars().skip(l).take(l).collect();
    let i = intersect(front, back);
    return char_to_value(i[0]);
}

fn char_to_value(c: char) -> usize {
    let v = c.clone() as usize;
    if v < 96 {
        return v - 38;
    } else {
        return v - 96;
    }
}

fn get_badge(line1: &str, line2: &str, line3: &str) -> usize {
    let i1 = intersect(line1.chars().collect(), line2.chars().collect());
    let i2 = intersect(i1, line3.chars().collect());
    return char_to_value(i2[0]);
}

fn intersect(v1: Vec<char>, v2: Vec<char>) -> Vec<char> {
    let h1:HashSet<char> = v1.into_iter().collect();
    let h2:HashSet<char> = v2.into_iter().collect();
    return h1.intersection(&h2).map(|c| c.clone()).collect();
}