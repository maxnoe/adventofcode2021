use crate::input;

fn parse_input(input: &String) -> Vec<i32> {
    input
        .trim()
        .lines()
        .map(str::parse::<i32>)
        .map(Result::unwrap)
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
    let mut current_sum: i32;
    let window = 3;

    let mut previous_sum: i32 = numbers[0..window].iter().sum();

    for i in 1..=(numbers.len() - window) {
        current_sum = numbers[i..i + window].iter().sum();

        if current_sum > previous_sum {
            n_larger += 1;
        }
        previous_sum = current_sum;
    }

    n_larger
}

pub fn day1() {
    let input = input::get_input(1);
    let numbers = parse_input(&input);
    println!("Part1: {}", part1(&numbers));
    println!("Part2: {}", part2(&numbers));
}
