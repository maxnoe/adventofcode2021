use crate::input;
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
struct Fish {
    timer: u8,
}

fn parse_input(input: &String) -> Vec<Fish> {
    input
        .trim()
        .split(",")
        .map(str::parse::<u8>)
        .map(Result::unwrap)
        .map(|timer| Fish{timer})
        .collect()
}

fn simulate_fishes(fishes: &Vec<Fish>, days: i32) -> usize {
    let mut counts = [0usize; 9];

    for fish in fishes {
        counts[fish.timer as usize] += 1;
    }

    for _ in 0..days {
        let last_counts = counts.clone();

        for timer in (0..8).rev() {
            counts[timer] = last_counts[timer + 1];
        }

        counts[8] = last_counts[0];
        counts[6] += last_counts[0];
    }

    counts.iter().sum()
}

fn part1(fishes: &Vec<Fish>) -> usize {
    simulate_fishes(&fishes, 80)
}

fn part2(fishes: &Vec<Fish>) -> usize {
    simulate_fishes(&fishes, 256)
}

pub fn day6() {
    let input = input::get_input(6);
    let lines = parse_input(&input);

    let t0 = Instant::now();
    println!("Part1: {}", part1(&lines));
    println!("Part2: {}", part2(&lines));
    println!("Time: {} us", t0.elapsed().as_micros());
}
