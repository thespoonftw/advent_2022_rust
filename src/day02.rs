use std::char;

use crate::problem::Problem;

pub struct Day02();

impl Problem for Day02 {

    fn part_one(&self, input: &str) -> String {
        let score = get_score(input, true);
        return score.to_string();
    }

    fn part_two(&self, input: &str) -> String {
        let score = get_score(input, false);
        return score.to_string();
    }
}


fn get_score(input: &str, is_part_1: bool) -> i32 {
        
    let mut total_score = 0;

    for line in input.lines() {
        
        let their_choice = line.chars().nth(0).unwrap();
        let my_info = line.chars().nth(2).unwrap();

        if is_part_1 {
            total_score += p1_win_score(their_choice, my_info);
            total_score += p1_info_score(my_info);            
        } else {
            total_score += p2_info_score(their_choice, my_info);
            total_score += p2_win_score(my_info);      
        }
    }

    return total_score;
}

fn p1_win_score(c1: char, c2: char) -> i32 {
    if (c1 == 'A' && c2 == 'X') || (c1 == 'B' && c2 == 'Y') || (c1 == 'C' && c2 == 'Z') {
        return 3;
    }
    else if (c1 == 'A' && c2 == 'Y') || (c1 == 'B' && c2 == 'Z') || (c1 == 'C' && c2 == 'X') {
        return 6;
    }
    else {
        return 0;
    }
}

fn p1_info_score(c: char) -> i32 {
    if c == 'X' {
        return 1;
    }
    else if c == 'Y' {
        return 2;
    }
    else if c == 'Z' {
        return 3;
    }
    return 0;
}

fn p2_win_score(c: char) -> i32 {
    if c == 'X' {
        return 0;
    }
    else if c == 'Y' {
        return 3;
    }
    else if c == 'Z' {
        return 6;
    }
    return 0;
}

fn p2_info_score(c1: char, c2: char) -> i32 {
    // rock
    if (c1 == 'A' && c2 == 'Y') || (c1 == 'B' && c2 == 'X') || (c1 == 'C' && c2 == 'Z') {
        return 1;
    }
    // paper
    else if (c1 == 'B' && c2 == 'Y') || (c1 == 'C' && c2 == 'X') || (c1 == 'A' && c2 == 'Z') {
        return 2;
    }
    // scissors
    else {
        return 3;
    }
}