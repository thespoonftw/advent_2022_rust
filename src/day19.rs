use crate::problem::Problem;

use std::cmp;

// prune any branches with fewer geodes at a given time than the best so far
// if a geode machine can be built, it must be built
// dont build more robots of a type than could be spent in a turn

pub struct Day19();

impl Problem for Day19 {

    fn part_one(&self, input: &str) -> String {

        let blueprints = parse_blueprints(input);

        let mut returner = 0;
        for i in 0..blueprints.len() {
            returner += find_best(blueprints[i], 24) * (i + 1);
        }
        return returner.to_string();
    }

    fn part_two(&self, input: &str) -> String {
        
        let blueprints = parse_blueprints(input);
        let len = cmp::min(blueprints.len(), 3);

        let mut returner = 1;
        for i in 0..len {
            returner *= find_best(blueprints[i], 32);
        }
        return returner.to_string();
    }
}

#[derive(Clone, Debug, Copy)]
struct Blueprint {
    max_orebots: usize,
    orebot_ore: usize,
    claybot_ore: usize,
    obsidbot_ore: usize,
    obsidbot_clay: usize,
    geobot_ore: usize,
    geobot_obsid: usize,
}

#[derive(Clone, Debug)]
struct FactoryState {
    time_remaining: usize,
    ore: usize,
    clay: usize,
    obsid: usize,
    geo: usize,
    orebots: usize,
    claybots: usize,
    obsidbots: usize,
    geobots: usize
}

fn parse_blueprints(input: &str) -> Vec<Blueprint> {
    return input.lines().map(|line| parse_blueprint(line)).collect();
}

fn parse_blueprint(input: &str) -> Blueprint {
    let indices = [6, 12, 18, 21, 27, 30];
    let split:Vec<&str> = input.split_ascii_whitespace().collect();
    let c:Vec<usize> = indices.iter().map(|i| split[*i].parse().unwrap()).collect();
    let max_orebots = [c[0], c[1], c[2], c[4]].iter().max().unwrap().clone();
    return Blueprint { max_orebots, orebot_ore: c[0], claybot_ore: c[1], obsidbot_ore: c[2], obsidbot_clay: c[3], geobot_ore: c[4], geobot_obsid: c[5] };
}

fn find_best(bp: Blueprint, duration: usize) -> usize {

    let mut factory_states:Vec<Vec<FactoryState>> = vec![Vec::<FactoryState>::new(); duration + 1];

    let first_factory = FactoryState{ 
        time_remaining: duration, ore: 0, clay: 0, obsid: 0, geo: 0, 
        orebots: 1, claybots: 0, obsidbots: 0, geobots: 0 
    };

    factory_states[duration].push(first_factory);
    
    for t in (1..=duration).rev() {

        let states = factory_states[t].clone();

        let new_states:Vec<FactoryState> = states.iter().map(|state| state.get_new_states(bp)).into_iter().flatten().collect();

        for s in new_states {
            let n = s.time_remaining;
            factory_states[n].push(s);
        }
    }
     
    return factory_states[0].clone().iter().max_by_key(|s| s.geo).unwrap().geo;

}

impl FactoryState {

    fn get_new_states(&self, bp: Blueprint) -> Vec<FactoryState> {

        let mut returner:Vec<FactoryState> = Vec::new();
    
        let geo_t = self.time_til_geobot(bp);

        // can make a geobot now! always the best option
        if geo_t.is_some() && geo_t.unwrap() == 0 {

            let mut f = self.clone();
            f.create_geobot(bp);
            returner.push(f);
            return returner;

        // can't make any more geobots, consider waiting til end
        } else if geo_t.is_none() && self.geobots > 0 {
            
            let mut f = self.clone();
            f.pass_time(self.time_remaining);
            returner.push(f);

        // otherwise, add as an option
        } else if geo_t.is_some() {

            let mut f = self.clone();
            f.pass_time(geo_t.unwrap());
            f.create_geobot(bp);
            returner.push(f);

        }

        let ore_t = self.time_til_orebot(bp);
        let clay_t = self.time_til_claybot(bp);
        let obsid_t = self.time_til_obsidbot(bp);

        if ore_t.is_some() {
            let mut f = self.clone();
            f.pass_time(ore_t.unwrap());
            f.create_orebot(bp);
            returner.push(f);
        }

        if clay_t.is_some() {
            let mut f = self.clone();
            f.pass_time(clay_t.unwrap());
            f.create_claybot(bp);
            returner.push(f);
        }

        if obsid_t.is_some() {
            let mut f = self.clone();
            f.pass_time(obsid_t.unwrap());
            f.create_obsbot(bp);
            returner.push(f);
        }

        return returner;

    }

    fn pass_time(&mut self, t: usize) {
        self.time_remaining -= t;
        self.ore += t * self.orebots;
        self.clay += t * self.claybots;
        self.obsid += t * self.obsidbots;
        self.geo += t * self.geobots;
    }

    fn create_orebot(&mut self, bp: Blueprint) {
        self.pass_time(1);
        self.ore -= bp.orebot_ore;
        self.orebots += 1;        
    }

    fn create_claybot(&mut self, bp: Blueprint) {
        self.pass_time(1);
        self.ore -= bp.claybot_ore;
        self.claybots += 1;        
    }

    fn create_obsbot(&mut self, bp: Blueprint) {
        self.pass_time(1);
        self.ore -= bp.obsidbot_ore;
        self.clay -= bp.obsidbot_clay;
        self.obsidbots += 1;        
    }

    fn create_geobot(&mut self, bp: Blueprint) {
        self.pass_time(1);
        self.ore -= bp.geobot_ore;
        self.obsid -= bp.geobot_obsid;
        self.geobots += 1;
    }

    fn time_til_orebot(&self, bp: Blueprint) -> Option<usize> {
        if self.orebots >= bp.max_orebots { return None }
        let t = self.time_til_ore(bp.orebot_ore);
        if self.time_remaining < t + 1 { return None; }
        return Some(t);
    }

    fn time_til_claybot(&self, bp: Blueprint) -> Option<usize> {
        if self.claybots >= bp.obsidbot_clay { return None }
        let t = self.time_til_ore(bp.claybot_ore); 
        if self.time_remaining < t + 1 { return None; }
        return Some(t);
    }

    fn time_til_obsidbot(&self, bp: Blueprint) -> Option<usize> {
        if self.claybots == 0 { return None }
        if self.obsidbots >= bp.geobot_obsid { return None }
        let t1 = self.time_til_ore(bp.obsidbot_ore);
        let t2 = self.time_til_clay(bp.obsidbot_clay);
        let t = cmp::max(t1, t2);
        if self.time_remaining < t + 1 { return None; }
        return Some(t);
    }

    fn time_til_geobot(&self, bp: Blueprint) -> Option<usize> {
        if self.obsidbots == 0 { return None }
        let t1 = self.time_til_ore(bp.geobot_ore);
        let t2 = self.time_til_obsid(bp.geobot_obsid);
        let t = cmp::max(t1, t2);
        if self.time_remaining < t + 1 { return None; }
        return Some(t);
    }

    fn time_til_ore(&self, ore: usize) -> usize {
        return if self.ore >= ore { 0 } else { ceil_div(ore - self.ore, self.orebots) };  
    }

    fn time_til_clay(&self, clay: usize) -> usize {
        return if self.clay >= clay { 0 } else { ceil_div(clay - self.clay, self.claybots) };  
    }

    fn time_til_obsid(&self, obsid: usize) -> usize {
        return if self.obsid >= obsid { 0 } else { ceil_div(obsid - self.obsid, self.obsidbots) };  
    }

}

fn ceil_div(a: usize, b: usize) -> usize {
    return ((a - 1) / b) + 1;
}
