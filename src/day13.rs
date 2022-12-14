use crate::problem::Problem;

pub struct Day13();

impl Problem for Day13 {

    fn part_one(&self, input: &str) -> String {

        let pairs:Vec<(Packet, Packet)> = input.lines()
            .collect::<Vec<&str>>()
            .chunks(3)
            .map(|lines| (Packet::new(lines[0]), Packet::new(lines[1])))
            .collect();

        let mut i = 1;
        let mut sum = 0;
        for pair in pairs {
            if is_order_correct(pair.0, pair.1) == Order::Correct {
                sum += i;
            }
            i += 1;
        }

        return sum.to_string();
    }

    fn part_two(&self, input: &str) -> String {

        let packets = input.lines()
            .filter(|s| s.len() > 0)
            .map(|s| Packet::new(s))
            .collect::<Vec<Packet>>();

        let decoder_1 = Packet{list: None, value: Some(6) };
        let decoder_2 = Packet{list: None, value: Some(2) };
        let mut value_1 = 0;
        let mut value_2 = 0;

        for p in packets {

            if is_order_correct(p.clone(), decoder_1.clone()) == Order::Correct {
                value_1 += 1;

                if is_order_correct(p.clone(), decoder_2.clone()) == Order::Correct {
                    value_2 += 1;
                }
            }
        }

        return ((value_1 + 2) * (value_2 + 1)).to_string();

    }
}

#[derive(PartialEq)]
enum Order {
    Unknown,
    Correct,
    Wrong,    
}

#[derive(Debug)]
#[derive(Clone)]
struct Packet {
    value: Option<usize>,
    list: Option<Vec<Packet>>
}

impl Packet {

    fn new(input: &str) -> Packet {

        if input.len() == 0 {
            return Packet{value: None, list: None};
        }
        else if input.starts_with('[') {
    
            let chars:Vec<char> = input.chars().collect();
    
            let mut vec = Vec::<Packet>::new();
    
            let mut start_packet:usize = 1;
            let mut level:usize = 0;
    
            for i in 1..input.len() {
    
                let c = chars[i];
                if c == '[' {
                    level += 1
                } else if (c == ',' || c == ']') && level == 0 {
                    let substr = &chars[start_packet..i];
                    let str:String = substr.iter().collect();
                    let packet = Packet::new(&str);
                    vec.push(packet);
                    start_packet = i + 1;
                } else if c == ']' {
                    level -= 1;
                }
            }
    
            return Packet{value: None, list: Some(vec)};
        } else {
    
            let value = Some(input.parse::<usize>().unwrap());
            return Packet{value, list: None};
        }
    }

    fn is_empty(&self) -> bool {
        return self.value.is_none() && self.list.is_none();
    }

}

fn is_order_correct(left: Packet, right: Packet) -> Order {

    // check for empty packets
    if left.is_empty() && right.is_empty() {
        return Order::Unknown;
    } else if left.is_empty() && !right.is_empty()  {
        return Order::Correct;
    } else if !left.is_empty() && right.is_empty() {
        return Order::Wrong
    }

    // compare values
    if !left.value.is_none() && !right.value.is_none() {
        let left_value = left.value.unwrap();
        let right_value = right.value.unwrap();

        if left_value < right_value {
            return Order::Correct;
        } else if left_value > right_value {
            return Order::Wrong;
        } else {
            return Order::Unknown;
        }
    }

    // compare lists
    let left_list = if left.list.is_none() { create_list(left.value.unwrap()) } else { left.list.unwrap() };
    let right_list = if right.list.is_none() { create_list(right.value.unwrap()) } else { right.list.unwrap() };

    let mut i = 0;
    loop {
        let left_over = i >= left_list.len();
        let right_over = i >= right_list.len();

        if left_over && right_over {
            return Order::Unknown;
        } else if left_over {
            return Order::Correct;
        } else if right_over {
            return Order::Wrong;
        }

        let left_packet = left_list.get(i).unwrap().clone();
        let right_packet = right_list.get(i).unwrap().clone();
        
        let result = is_order_correct(left_packet, right_packet);
        if result != Order::Unknown {
            return result;
        }
        i += 1;
    }
}

fn create_list(v: usize) -> Vec<Packet> {
    let mut returner = Vec::new();
    returner.push(Packet { list: None, value: Some(v) });
    return returner;
}