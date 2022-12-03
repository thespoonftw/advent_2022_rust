mod day01; mod day02; mod day03; mod day04; mod day05;
mod day06; mod day07; mod day08; mod day09; mod day10;
mod day11; mod day12; mod day13; mod day14; mod day15;
mod day16; mod day17; mod day18; mod day19; mod day20;
mod day21; mod day22; mod day23; mod day24; mod day25;
mod problem;

use std::fs;

fn main() {

    run(3);

}

fn run(index: usize) {

    let mut v:Vec<&dyn problem::Problem> = Vec::new();
    v.push(&day01::Day01()); // index 0
    v.push(&day01::Day01()); v.push(&day02::Day02()); v.push(&day03::Day03()); v.push(&day04::Day04()); v.push(&day05::Day05());
    v.push(&day06::Day06()); v.push(&day07::Day07()); v.push(&day08::Day08()); v.push(&day09::Day09()); v.push(&day10::Day10());
    v.push(&day11::Day11()); v.push(&day12::Day12()); v.push(&day13::Day13()); v.push(&day14::Day14()); v.push(&day15::Day15());
    v.push(&day16::Day16()); v.push(&day17::Day17()); v.push(&day18::Day18()); v.push(&day19::Day19()); v.push(&day20::Day20());
    v.push(&day21::Day21()); v.push(&day22::Day22()); v.push(&day23::Day23()); v.push(&day24::Day24()); v.push(&day25::Day25());

    let day = v.get(index).unwrap();
    let num = format!("{:02}", index);

    println!("== Day{num} Test ==");
    let test_path = format!("../input/test{num}.txt");
    let test_string = fs::read_to_string(test_path).expect("Failed to read file");
    let t1 = day.part_one(&test_string);
    println!("Part 1: {t1}");
    let t2 = day.part_two(&test_string);
    println!("Part 2: {t2}");

    println!("== Day{num} ==");
    let input_path = format!("../input/input{num}.txt");
    let input_string = fs::read_to_string(input_path).expect("Failed to read file");
    let i1 = day.part_one(&input_string);
    println!("Part 1: {i1}");
    let i2 = day.part_two(&input_string);
    println!("Part 2: {i2}");
}