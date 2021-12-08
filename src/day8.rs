use itertools::Itertools;
use crate::input;
use std::time::Instant;
use std::collections::{HashMap,HashSet};


#[derive(Debug)]
struct DisplayConfig {
    unique_patterns: Vec<String>,
    displayed_values: Vec<String>
}

fn parse_display(line: &str) -> DisplayConfig {
    let mut split = line.split(" | ");

    let unique_patterns = split
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.chars().sorted().collect::<String>())
        .collect();

    let displayed_values = split
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.chars().sorted().collect::<String>())
        .collect();

    DisplayConfig{unique_patterns, displayed_values}

}

fn parse_input(input: &String) -> Vec<DisplayConfig> {
    input
        .lines()
        .map(parse_display)
        .collect()
}

fn part1(positions: &Vec<DisplayConfig>) -> i32 {
    let unique_lens: HashMap<u8, u8> = [
        (2, 1),
        (4, 4),
        (3, 7),
        (7, 8),
    ].iter().cloned().collect();

    let mut n_unique = 0;
    for position in positions {
        for value in &position.displayed_values {
            if unique_lens.contains_key(&(value.len() as u8)) {
                n_unique += 1
            }
        }
    }

    n_unique
}

fn find_value_with_len(patterns: &Vec<String>, len: usize) -> HashSet<char>{
    patterns
        .iter()
        .find(|s| s.len() == len)
        .unwrap()
        .chars()
        .collect()
}

fn determine_value(display: &DisplayConfig) -> i32 {
    let one = find_value_with_len(&display.unique_patterns, 2);
    let four = find_value_with_len(&display.unique_patterns, 4);

    let mut value = 0;
    for (i, number) in display.displayed_values.iter().enumerate() {
        let set: HashSet<char> = number.chars().collect();
        let length = number.len();
        let intersect_one = one.intersection(&set).count();
        let intersect_four = four.intersection(&set).count();

        let digit = match (length, intersect_one, intersect_four) {
            (2, _, _) => 1,
            (3, _, _) => 7,
            (4, _, _) => 4,
            (7, _, _) => 8,

            (5, 2, _) => 3,
            (5, 1, 2) => 2,
            (5, 1, 3) => 5,

            (6, 1, _) => 6,
            (6, 2, 3) => 0,
            (6, 2, 4) => 9,

            _ => panic!("Unexpected pattern")
        };

        value += 10i32.pow((3 - i) as u32) * digit;
    }

    value
}

fn part2(displays: &Vec<DisplayConfig>) -> i32 {
    displays.iter().map(determine_value).sum()
}

pub fn day8() {
    let input = input::get_input(8);
    let displays = parse_input(&input);

    let t0 = Instant::now();
    println!("Part1: {}", part1(&displays));
    println!("Part2: {}", part2(&displays));
    println!("Time: {} us", t0.elapsed().as_micros());
}
