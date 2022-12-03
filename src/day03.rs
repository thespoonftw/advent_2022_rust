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
    let front:HashSet<char> = line.chars().take(l).collect();
    let back:HashSet<char> = line.chars().skip(l).take(l).collect();
    let i:HashSet<&char> = front.intersection(&back).collect();
    let c = i.iter().next().unwrap().clone().clone();
    return char_to_value(c);
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
    let l1:HashSet<char> = line1.chars().collect();
    let l2:HashSet<char> = line2.chars().collect();
    let l3:HashSet<char> = line3.chars().collect();
    let i1:HashSet<&char> = l1.intersection(&l2).collect();
    let i2:HashSet<&char> = l1.intersection(&l3).collect();
    let i3:HashSet<&&char> = i1.intersection(&i2).collect();
    let c = i3.iter().next().unwrap().clone().clone().clone();
    return char_to_value(c);
}