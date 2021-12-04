use crate::input;
use std::collections::VecDeque;
use std::time::Instant;


#[derive(Debug)]
#[derive(Clone, Copy)]
struct Board {
    numbers: [[u8; 5]; 5],
    marked: [[bool; 5]; 5],
}

impl Board {
    pub fn new() -> Self {
        Self{
            numbers: [[0u8; 5]; 5],
            marked: [[false; 5]; 5],
        }
    }

    pub fn finished(&self) -> bool {
        for row in 0..5 {
            if self.marked[row].iter().all(|m| *m) {
                return true;
            }
        }

        'col: for col in 0..5 {
            for row in 0..5 {
                if !self.marked[row][col] {
                    continue 'col;
                }
            }
            return true;
        }

        false
    }

    pub fn score(&self) -> u32 {
        let mut sum = 0u32;
        for row in 0..5 {
            for col in 0..5 {
                if !self.marked[row][col] {
                    sum += self.numbers[row][col] as u32;
                }
            }
        }
        sum
    }

    pub fn mark(&mut self, number: u8) -> bool {
        for row in 0..5 {
            match self.numbers[row].iter().position(|n| *n == number) {
                Some(n) => {
                    self.marked[row][n] = true;
                    return true;
                },
                None => {},
            }
        }

        false
    }
}

fn parse_input(input: &String) -> (Vec<u8>, Vec<Board>) {
    let mut boards = Vec::new();

    let mut groups = input.split("\n\n");

    let numbers: Vec<u8> = groups
        .next()
        .unwrap()
        .split(",")
        .map(str::parse::<u8>)
        .map(Result::unwrap)
        .collect();

    for group in groups {
        let mut board = Board::new();
        group.split_ascii_whitespace()
            .map(str::parse::<u8>)
            .map(Result::unwrap)
            .enumerate()
            .for_each(|el: (usize, u8)| board.numbers[el.0 / 5][el.0 % 5] = el.1)
        ;

        boards.push(board);
    }
    
    (numbers, boards)
}


fn part1(numbers: &Vec<u8>, boards: &Vec<Board>) -> u32 {
    let mut boards = boards.clone();

    for number in numbers {
        for board in boards.iter_mut() {
            if board.mark(*number) {
                if board.finished() {
                    return board.score() * *number as u32;
                }
            }
        }
    }
    0
}


fn part2(numbers: &Vec<u8>, boards: &Vec<Board>) -> u32 {
    let mut boards: VecDeque<Board> = boards.iter().copied().collect();

    for number in numbers {
        for _ in 0..boards.len() {
            let mut board = boards.pop_front().unwrap();
            if board.mark(*number) && board.finished() {
                if boards.len() == 0 {
                    return board.score() * *number as u32;
                }
            } else {
                boards.push_back(board);
            }
        }
    }
    0
}


pub fn day4() {
    let input = input::get_input(4);
    let (numbers, boards) = parse_input(&input);

    let t0 = Instant::now();
    println!("Part1: {}", part1(&numbers, &boards));
    println!("Part2: {}", part2(&numbers, &boards));
    println!("Time: {} us", t0.elapsed().as_micros());
}
