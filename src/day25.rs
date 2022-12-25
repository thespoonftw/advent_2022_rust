use crate::problem::Problem;

pub struct Day25();

impl Problem for Day25 {

    fn part_one(&self, input: &str) -> String {
        let x:i64 = input.lines().into_iter().map(|l| to_decimal(l)).sum();
        return to_snafu(x);
    }

    fn part_two(&self, _input: &str) -> String {
        return "N/A".to_string();
    }
}

fn to_decimal(line: &str) -> i64 {

    let mut returner = 0;
    for (i, c) in line.chars().rev().enumerate() {
        let power = index_to_power(i);
        let value = power * char_to_value(c);
        returner += value;
    }
    return returner;

}

fn char_to_value(c: char) -> i64 {
    if c == '-' {
        return -1;
    } else if c == '=' {
        return -2;
    } else {
        return c.to_digit(10).unwrap() as i64;
    }
}

fn index_to_power(i: usize) -> i64 {
    let exp:u32 = i as u32;
    return (5 as i64).pow(exp);
}

fn to_snafu(mut num: i64) -> String { 

    let mut returner = Vec::new();
    while num > 0 {
        let digit = (num % 5) as u32;
        if digit < 3 {
            returner.push(std::char::from_digit(digit, 10).unwrap());
        } else {
            if digit == 3 {
                returner.push('=');
                num += 2;
            } else {
                returner.push('-');
                num += 1;
            }
        }
        num /= 5;
    }
    returner.reverse();
    return returner.into_iter().collect();

}