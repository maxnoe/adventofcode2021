use crate::input;
use std::collections::VecDeque;


fn parse_input(input: &String) -> Vec<i32> {
    input
        .trim()
        .split_ascii_whitespace()
        .map(|line: &str| line.parse::<i32>().expect("Failed to parse input"))
        .collect()
}

fn part1(numbers: &Vec<i32>) -> i32 {
    let mut n_larger = 0;
    let mut iter = numbers.iter();
    let mut before = iter.next().expect("Expected at least one number");

    for number in iter {
        if number > before {
            n_larger += 1;
        }
        before = number;
    }

    n_larger
}


fn part2(numbers: &Vec<i32>) -> i32 {
    let mut n_larger = 0;
    let mut iter = numbers.iter();

    let mut deq = VecDeque::from([
        iter.next().expect(""),
        iter.next().expect(""),
        iter.next().expect("")
    ]);
    let mut previous_sum: i32 = deq.iter().map(|x| *x).sum();
    let mut current_sum: i32;


    for number in iter {
        deq.pop_front();
        deq.push_back(number);

        current_sum = deq.iter().map(|x| *x).sum();

        if current_sum > previous_sum {
            n_larger += 1;
        }
        previous_sum = current_sum;
    }
    // deq.iter().sum();

    n_larger
}


pub fn day1() {
    let input = input::get_input(1);
    let numbers = parse_input(&input);
    println!("Part1: {}", part1(&numbers));
    println!("Part2: {}", part2(&numbers));
}
