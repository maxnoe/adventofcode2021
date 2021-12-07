use crate::input;
use std::time::Instant;


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
    let start = positions.iter().min().copied().unwrap_or(0);
    let end = positions.iter().max().copied().unwrap_or(0);

    let mut best_fuel = i32::max_value();
    for pos in start..=end {
        let fuel = total_fuel_consumption(&positions, pos);
        if fuel < best_fuel {
            best_fuel = fuel;
        }

    }

    best_fuel

}

pub fn day7() {
    let input = input::get_input(7);
    let lines = parse_input(&input);

    let t0 = Instant::now();
    println!("Part1: {}", part1(&lines));
    println!("Part2: {}", part2(&lines));
    println!("Time: {} us", t0.elapsed().as_micros());
}
