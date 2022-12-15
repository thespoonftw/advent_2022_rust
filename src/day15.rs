use crate::problem::Problem;

pub struct Day15();

impl Problem for Day15 {

    fn part_one(&self, input: &str) -> String {
        let zone = BeaconExclusionZone::parse(input);
        return zone.count_taken_space_in_row().to_string();
    }

    fn part_two(&self, input: &str) -> String {
        let zone = BeaconExclusionZone::parse(input);
        return zone.find_tuning_frequency().to_string();
    }
}

#[derive(Debug)]
#[derive(Clone)]
struct Sensor {
    x_pos: i32,
    y_pos: i32,
    x_beacon: i32,
    y_beacon: i32,
    range: i32,
}

impl Sensor {
    fn parse(input: &str) -> Sensor {
        let split = input.split_whitespace().collect::<Vec<&str>>();
        let x_pos = Sensor::parse_int(split[2]);
        let y_pos = Sensor::parse_int(split[3]);
        let x_beacon = Sensor::parse_int(split[8]);
        let y_beacon = Sensor::parse_int(split[9]);
        let range = (x_pos - x_beacon).abs() + (y_pos - y_beacon).abs();
        return Sensor {x_pos, y_pos, x_beacon, y_beacon, range }
    }

    fn parse_int(input: &str) -> i32 {
        return input.replace(&['x', 'y', ',', '=', ':'][..], "").parse().unwrap();
    }

    fn is_in_range(&self, point: (i32, i32)) -> bool {
        let dx = (self.x_pos - point.0).abs();
        let dy = (self.y_pos - point.1).abs();
        return dx + dy <= self.range;
    }

    fn find_outline(&self) -> Vec<(i32, i32)> {
        let mut returner = Vec::<(i32, i32)>::new();
        let r = self.range + 1;
        for i in 0..r {
            let x1 = self.x_pos + i;
            let y1 = self.y_pos + r - i;
            let x2 = self.x_pos + r - i;
            let y2 = self.y_pos - i;
            let x3 = self.x_pos - i;
            let y3 = self.y_pos - r + i;
            let x4 = self.x_pos - r + i;
            let y4 = self.y_pos + i;
            returner.push((x1, y1));
            returner.push((x2, y2));
            returner.push((x3, y3));
            returner.push((x4, y4));
        }
        return returner;
    }
}

#[derive(Debug)]
struct BeaconExclusionZone {
    row_number: i32,
    search_limit: i32,
    sensors: Vec<Sensor>,
}

impl BeaconExclusionZone {

    fn parse(input: &str) -> BeaconExclusionZone {
        
        // I manually added these values to the input data
        let row_number = input.lines().nth(0).unwrap().parse::<i32>().unwrap();
        let search_limit = input.lines().nth(1).unwrap().parse::<i32>().unwrap();
    
        let sensors = input.lines().skip(2).collect::<Vec<&str>>().iter().map(|line|
            Sensor::parse(line)
        ).collect::<Vec<Sensor>>();

        return BeaconExclusionZone { row_number, search_limit, sensors }
    }

    fn count_taken_space_in_row(&self) -> usize {

        // could find values from data instead of hard coding here
        let mid = 10_000_000;
        let max = 10_000_000;
        let mut row = vec![false; (mid + max) as usize];
    
        // fill in row with ranges
        for sensor in &self.sensors {
            let dx = sensor.range - (sensor.y_pos - self.row_number).abs();
            if dx < 0 { continue; }
            let min = sensor.x_pos - dx;
            let max = sensor.x_pos + dx;
            for x in min..=max {
                row[(x + mid) as usize] = true;
            }
        }
    
        // remove beacons from row
        for sensor in &self.sensors {
            if sensor.y_beacon == self.row_number {
                row[(sensor.x_beacon + mid) as usize] = false;
            }
        }
    
        // count true values
        let mut i = 0;
        for b in row { if b { i += 1; } }
        return i;

    }

    fn find_tuning_frequency(&self) -> usize {

        // for each sensor
        for s1 in 0..self.sensors.len() {
            let sensor1 = self.sensors[s1].clone();

            // for each point in the sensor's outline
            for p in sensor1.find_outline() {

                // ignore points out of bounds
                if p.0 < 0 || p.0 > self.search_limit { continue; }
                if p.1 < 0 || p.1 > self.search_limit { continue; }

                // check if its inside another sensor's range
                let mut checker = false;
                for s2 in 0..self.sensors.len() {
                    if s1 == s2 { continue; }
                    let sensor2 = self.sensors[s2].clone();
                    if sensor2.is_in_range(p) {
                        checker = true;
                        break;
                    }
                }

                // if not, its the one
                if !checker {
                    return (p.0 as usize) * 4000000 + (p.1 as usize);
                }
            }
        }

        // not found
        return 0;
    }

}



