use crate::input;
use std::time::Instant;
use phf::{Map, phf_map};

type Input = Vec<String>;


const CLOSING_PAIR: Map<char, char> = phf_map! {
    '{' => '}',
    '[' => ']',
    '<' => '>',
    '(' => ')',
};


fn parse_input(input: &String) -> Input {
    input
        .lines()
        .map(String::from)
        .collect()
}


fn score(invalid_char: Option<char>) -> i64 {
    match invalid_char {
        Some(')') => 3,
        Some(']') => 57,
        Some('}') => 1197,
        Some('>') => 25137,
        None => 0,
        _ => panic!("Invalid invalid char {}", invalid_char.unwrap())
    }
}


fn find_invalid_char(line: &String) -> Option<char> {
    let mut stack: Vec<char> = Vec::new();
    for chr in line.chars() {
        match chr {
            '(' | '{' | '[' | '<' => stack.push(chr),
            ')' | '}' | ']' | '>' => {
                let last = *stack.last().unwrap_or(&'\0');
                if chr == *CLOSING_PAIR.get(&last).unwrap_or(&'\0') {
                    stack.pop();
                } else {
                    return Some(chr);
                }
            },
            _ => panic!("Unexpected character")
        }
    }

    None
}


fn completion_score(line: &String) -> i64 {
    let mut stack: Vec<char> = Vec::new();

    for chr in line.chars() {
        match chr {
            '(' | '{' | '[' | '<' => stack.push(chr),
            ')' | '}' | ']' | '>' => {
                let last = *stack.last().unwrap_or(&'\0');
                if chr == *CLOSING_PAIR.get(&last).unwrap_or(&'\0') {
                    stack.pop();
                } else {
                   panic!("Corrupted line in part2");
                }
            },
            _ => panic!("Unexpected character {}", chr)
        }
    }

    let mut score = 0;
    stack.reverse();
    println!("{} {}", line, stack.iter().map(|c| CLOSING_PAIR.get(c).unwrap_or(&'\0')).collect::<String>());
    for chr in stack {
        score *= 5;
        score += match chr {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => panic!("Unexpected character {}", chr)
        };
    }

    score
}


fn part1(lines: &Input) -> i64 {
    lines.iter()
        .map(find_invalid_char)
        .map(score)
        .sum()
}

fn part2(lines: &Input) -> i64 {
    let mut scores: Vec<i64> = lines.iter()
        .filter(|l| find_invalid_char(l).is_none())
        .map(completion_score)
        .collect();

    scores.sort();
    scores[scores.len() / 2]

}

pub fn day10() {
    let input = input::get_input(10);
    let grid = parse_input(&input);

    let t0 = Instant::now();
    println!("Part1: {}", part1(&grid));
    println!("Part2: {}", part2(&grid));
    println!("Time: {} us", t0.elapsed().as_micros());
}
