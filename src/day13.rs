use crate::input;
use std::time::Instant;
use std::collections::HashSet;
use itertools::Itertools;

enum Axis {
    X,
    Y
}


struct Input {
    dots: Vec<(u32, u32)>,
    folds: Vec<(Axis, u32)>,
}

fn parse_input(input: &String) -> Input {
    let mut iter = input.split("\n\n");
    let dots = iter
        .next().unwrap()
        .lines()
        .map(|l| l.split(",").map(str::parse).map(Result::unwrap).collect_tuple().unwrap())
        .collect();

    let folds = iter
        .next().unwrap()
        .lines()
        .map(|l| {
            let mut iter = l.split("=");
            let axis = match iter.next().unwrap().chars().last() {
                Some('x') => Axis::X,
                Some('y') => Axis::Y,
                _ => panic!("Could not parse axis"),
            };
            let coord = iter.next().unwrap().parse().unwrap();
            (axis, coord)
        })
        .collect();

    return Input{dots, folds};
}


fn apply_fold(grid: &HashSet<(u32, u32)>, axis: &Axis, position: u32) -> HashSet<(u32, u32)>{
    let mut new_grid = HashSet::new();

    for (row, col) in grid.iter().copied() {
        match axis {
            Axis::X => {
                if row < position {
                    new_grid.insert((row, col));
                } else {
                    let new_row = position - (row - position);
                    // println!("Folding {} {} -> {} {}", row, col, new_row, col);
                    new_grid.insert((new_row, col));
                }
            },
            Axis::Y => {
                if col < position {
                    new_grid.insert((row, col));
                } else {
                    let new_col = position - (col - position);
                    // println!("Folding {} {} -> {} {}", row, col, row, new_col);
                    new_grid.insert((row, new_col));
                }
            },
        }
    }

    new_grid
}

fn part1(input: &Input) -> usize {
    let mut grid: HashSet<(u32, u32)> = input.dots.iter().copied().collect();
    let (axis, position) = input.folds.first().unwrap();
    grid = apply_fold(&mut grid, axis, *position);
    grid.len()
}

fn part2(input: &Input) {
    let mut grid: HashSet<(u32, u32)> = input.dots.iter().copied().collect();
    for (axis, position) in &input.folds {
        grid = apply_fold(&mut grid, axis, *position);
    }

    let n_rows = grid.iter().map(|(r, _)| *r).max().unwrap() + 1;
    let n_cols = grid.iter().map(|(_, c)| *c).max().unwrap() + 1;
    let mut display = vec![vec!['.'; n_rows as usize]; n_cols as usize];

    for (r, c) in grid {
        display[c as usize][r as usize] = '#';
    }

    for row in display {
        println!("{}", row.iter().copied().join(""));
    }
}

pub fn day13() {
    let input = input::get_input(13);
    let manual = parse_input(&input);
    println!("{}", manual.dots.len());

    let t0 = Instant::now();
    println!("Part1: {}", part1(&manual));
    println!("Part2:");
    part2(&manual);
    println!("Time: {} us", t0.elapsed().as_micros());
}
