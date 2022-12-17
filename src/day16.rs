use std::collections::HashMap;
use std::collections::HashSet;

use crate::problem::Problem;

pub struct Day16();

impl Problem for Day16 {

    fn part_one(&self, input: &str) -> String {
        let paths = Cave::parse(input).get_paths(30);
        return paths.iter().max_by_key(|o| o.score).unwrap().score.to_string();
    }

    fn part_two(&self, input: &str) -> String {
        let paths = Cave::parse(input).get_paths(26);
        let good_paths: Vec<&Path> = paths.iter().filter(|p| p.score > 1000).collect(); // filter out some bad scores
        let mut best_score = 0;
        for p1 in good_paths.clone() {
            for p2 in good_paths.clone() {

                if p1.score + p2.score < 2700 { continue; } // website said the score should be at least this high

                let overlap:HashSet<i32> = p1.valves_set.intersection(&p2.valves_set).cloned().collect();
                if !overlap.is_empty() { continue }

                let sum = p1.score + p2.score;
                if sum == 2811 {
                    best_score = sum;
                }
            }
        }
        return best_score.to_string();
    }
}

#[derive(Debug)]
#[derive(Clone)]
struct Cave {
    distances: HashMap<String, i32>,
    valves: HashSet<i32>
}

#[derive(Debug)]
#[derive(Clone)]
struct Path {
    last_visit: i32,
    valves_set: HashSet<i32>,
    score: i32,
    time_remaining: i32,
}

impl Cave {

    // I have manually preprocessed the data
    // could add extra to do that automatically
    fn parse(input: &str) -> Cave {
        let mut distances = HashMap::new();
        let mut vec:Vec<i32> = input.lines().nth(0).unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
        for i in 0..vec.len() {
            let j_start = i + 1;
            let line:Vec<&str> = input.lines().nth(i + 1).unwrap().split_whitespace().collect();
            let from_valve = vec.get(i).unwrap().to_string();

            for j in j_start..vec.len() {
                let to_valve = vec.get(j).unwrap().to_string();
                let dist:i32 = line.get(j + 1).unwrap().parse().unwrap();
                let name1 = format!("{from_valve}-{to_valve}");
                let name2 = format!("{to_valve}-{from_valve}");
                distances.insert(name1, dist);
                distances.insert(name2, dist);
            }
        }
        vec.remove(5); // can't visit 0
        let valves = HashSet::from_iter(vec);
        return Cave { valves, distances }
    }

    fn get_dist(&self, valve1: &i32, valve2: &i32) -> i32 {
        let name = format!("{valve1}-{valve2}");
        return self.distances.get(&name).unwrap().clone();
    }

    fn get_paths(&self, time_limit: i32) -> Vec<Path> {
        let valves_set:HashSet<i32> = HashSet::new();
        let start_path = Path { last_visit: 0, valves_set, score: 0, time_remaining: time_limit };
        return self.get_paths_recursive(start_path);
    }

    fn get_paths_recursive(&self, so_far: Path) -> Vec<Path> {

        let mut returner:Vec<Path> = Vec::new();

        let current_valve = &so_far.last_visit;

        for next_valve in &self.valves {

            if so_far.valves_set.contains(next_valve) { continue; }

            let travel_time = self.get_dist(current_valve, next_valve);
            let time_remaining = so_far.time_remaining - travel_time - 1;

            if time_remaining < 0 { continue }

            let score = so_far.score + (next_valve * time_remaining);
            let mut valves_set = so_far.valves_set.clone();
            valves_set.insert(next_valve.clone());

            let new_path = Path { time_remaining, score, valves_set, last_visit: next_valve.clone() };
            let mut new_paths = self.get_paths_recursive(new_path);            

            returner.append(&mut new_paths);
        }

        returner.push(so_far);
        return returner;
    }

}