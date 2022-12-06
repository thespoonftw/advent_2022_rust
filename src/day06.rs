use crate::problem::Problem;
use std::collections::HashSet;

pub struct Day06();

impl Problem for Day06 {

    fn part_one(&self, input: &str) -> String {
        return find_key(input, 4)
    }

    fn part_two(&self, input: &str) -> String {
        return find_key(input, 14)
    }
}

fn find_key(input: &str, length: usize) -> String {

    let chars = input.chars().collect::<Vec<char>>();

    for i in length..chars.len() {
        let set:HashSet<char> = chars[i-length..i].to_vec().into_iter().collect();
        if set.len() == length {
            return i.to_string();
        }
    }

    return "".to_string();

}