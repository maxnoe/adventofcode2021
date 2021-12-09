use crate::input;
use std::time::Instant;
use std::cmp::min;


fn parse_input(input: &String) -> Vec<i32> {
    input
        .trim()
        .split(",")
        .map(str::parse::<i32>)
        .map(Result::unwrap)
        .collect()
}

fn median(numbers: &Vec<i32>) -> i32 {
    let mut numbers = numbers.clone();
    numbers.sort();
    numbers[numbers.len() / 2]
}

fn mean(numbers: &Vec<i32>) -> i32 {
    let sum: f64 = numbers.iter().map(|n| *n as f64).sum();
    (sum / numbers.len() as f64) as i32
}

fn part1(positions: &Vec<i32>) -> i32 {
    let m = median(&positions);

    positions
        .iter()
        .map(|p| (*p - m).abs())
        .sum()
}


fn fuel_consuption(pos1: i32, pos2: i32) -> i32 {
    let steps = (pos1 - pos2).abs();
    (steps * (steps + 1)) / 2
}

fn total_fuel_consumption(positions: &Vec<i32>, target_position: i32) -> i32 {
    positions.iter()
        .map(|p| fuel_consuption(*p, target_position))
        .sum()
}

fn part2(positions: &Vec<i32>) -> i32 {
    let pos1 = mean(&positions);
    let fuel1 = total_fuel_consumption(positions, pos1);
    let fuel2 = total_fuel_consumption(positions, pos1 + 1);
    println!("{} {}", fuel1, fuel2);
    min(fuel1, fuel2)
}

pub fn day7() {
    let input = input::get_input(7);
    let lines = parse_input(&input);

    let t0 = Instant::now();
    println!("Part1: {}", part1(&lines));
    println!("Part2: {}", part2(&lines));
    println!("Time: {} us", t0.elapsed().as_micros());
}
