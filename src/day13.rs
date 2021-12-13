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

    for (x, y) in grid.iter().copied() {
        match axis {
            Axis::X => {
                if x < position {
                    new_grid.insert((x, y));
                } else {
                    let new_x = position - (x - position);
                    // println!("Folding {} {} -> {} {}", x, y, new_x, y);
                    new_grid.insert((new_x, y));
                }
            },
            Axis::Y => {
                if y < position {
                    new_grid.insert((x, y));
                } else {
                    let new_y = position - (y - position);
                    // println!("Folding {} {} -> {} {}", x, y, x, new_y);
                    new_grid.insert((x, new_y));
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

    let n_cols = grid.iter().map(|(x, _)| *x).max().unwrap() + 1;
    let n_rows = grid.iter().map(|(_, y)| *y).max().unwrap() + 1;
    let mut display = vec![vec![' '; n_cols as usize]; n_rows as usize];

    for (x, y) in grid {
        display[y as usize][x as usize] = '▇';
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
