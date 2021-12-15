use crate::input;
use std::time::Instant;
use std::collections::HashMap;
use itertools::{Itertools,MinMaxResult};

type Pair = (char, char);

struct Input {
    polymer_template: Vec<char>,
    insertion_rules: HashMap<Pair, (Pair, Pair)>,
}

fn parse_input(input: &String) -> Input {
    let mut iter = input.split("\n\n");
    let polymer_template = iter.next().unwrap().trim().chars().collect();

    let insertion_rules = iter
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let mut iter = l.split(" -> ");
            let pair: Pair = iter.next().unwrap().chars().collect_tuple().unwrap();
            let replacement = iter.next().unwrap().chars().next().unwrap();
            (pair, ((pair.0, replacement), (replacement, pair.1)))
        })
        .collect();

    Input{polymer_template, insertion_rules}

}

fn pair_insertion(counter: &HashMap<Pair, i64>, rules: &HashMap<Pair, (Pair, Pair)>) -> HashMap<Pair, i64> {
    let mut new_counter = HashMap::new();

    for (pair, count) in counter {
        let (left, right) = rules[pair];
        *new_counter.entry(left).or_insert(0) += count;
        *new_counter.entry(right).or_insert(0) += count;
    }

    new_counter
}

fn polymer_development(input: &Input, n_steps: u16) -> i64 {
    let mut counter = HashMap::new();
    for i in 0..(input.polymer_template.len() - 1) {
        let pair = (input.polymer_template[i], input.polymer_template[i + 1]);
        *counter.entry(pair).or_insert(0) += 1;
    }

    for _ in 0..n_steps {
        counter = pair_insertion(&counter, &input.insertion_rules);
    }

    let mut char_counter: HashMap<char, i64> = HashMap::new();
    for (pair, count) in &counter {
        *char_counter.entry(pair.0).or_insert(0) += count;
    }
    *char_counter.entry(*input.polymer_template.last().unwrap()).or_insert(0) += 1;


    let minmax = char_counter.iter().minmax_by_key(|(_, count)| *count);

    match minmax {
        MinMaxResult::MinMax(min, max) => max.1 - min.1,
        _ => panic!("Could not find minmax"),
    }
}


fn part1(input: &Input) -> i64 {
    polymer_development(input, 10)
}

fn part2(input: &Input) -> i64 {
    polymer_development(input, 40)
}

pub fn day14() {
    let input = input::get_input(14);
    let polymer_rules = parse_input(&input);

    let t0 = Instant::now();
    println!("Part1: {}", part1(&polymer_rules));
    println!("Part2: {}", part2(&polymer_rules));
    part2(&polymer_rules);
    println!("Time: {} us", t0.elapsed().as_micros());
}
