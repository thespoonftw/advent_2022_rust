use crate::problem::Problem;
use std::collections::HashMap;

pub struct Day02();

impl Problem for Day02 {

    fn part_one(&self, input: &str) -> String {
        return Scorer::new_p1_scorer().get_total_score(input);
    }

    fn part_two(&self, input: &str) -> String {
        return Scorer::new_p2_scorer().get_total_score(input);
    }
}

struct Scorer {
    hash: HashMap<String, i32>
}

impl Scorer {

    fn new_p1_scorer() -> Scorer {
        let mut s = Scorer{hash: HashMap::new()}; 
        // rock
        s.add("C X", 7); // win
        s.add("A X", 4); // tie
        s.add("B X", 1); // lose
        // paper
        s.add("A Y", 8); // win 
        s.add("B Y", 5); // tie
        s.add("C Y", 2); // lose
        // scissors
        s.add("B Z", 9); // win
        s.add("C Z", 6); // tie
        s.add("A Z",3); // lose
        return s;
    }

    fn new_p2_scorer() -> Scorer {
        let mut s = Scorer{hash: HashMap::new()}; 
        // lose
        s.add("B X", 1); // rock
        s.add("C X", 2); // paper
        s.add("A X", 3); // scis
        // tie
        s.add("A Y", 4); // rock 
        s.add("B Y", 5); // paper
        s.add("C Y", 6); // scis
        // win
        s.add("C Z", 7); // rock
        s.add("A Z", 8); // paper
        s.add("B Z", 9); // scis
        return s;
    }

    fn add(&mut self, s: &str, i: i32) {
        self.hash.insert(s.to_string(), i);
    }

    fn get_score(&self, input: &str) -> i32 {
        return self.hash.get(input).unwrap().clone();
    }

    fn get_total_score(&self, input: &str) -> String {
        let sum:i32 = input.lines().map(|l| self.get_score(l)).sum();
        return sum.to_string();
    }

}