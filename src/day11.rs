use crate::input;
use std::time::Instant;
use std::collections::VecDeque;

type Input = Vec<Vec<u16>>;

const N_ROWS: usize = 10;
const N_COLS: usize = 10;

const NEIGHBORS: [(i32, i32); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1), (0, 1),
    (1, -1), (1, 0), (1, 1),
];


fn parse_input(input: &String) -> Input {
    input
        .lines()
        .map(|l| l.chars().map(|c| c as u16 - b'0' as u16).collect())
        .collect()
}


fn do_step(grid: &mut Input) -> i64 {
    let mut n_flashes = 0;

    let mut has_flashed = [[false; N_COLS]; N_ROWS];
    let mut to_check = VecDeque::new();

    for row in 0..N_ROWS {
        for col in 0..N_COLS {
            grid[row][col] += 1;

            if grid[row][col] > 9 {
                n_flashes += 1;
                has_flashed[row][col] = true;
                to_check.push_back((row, col));
            }
        }
    }

    while to_check.len() > 0 {
        let (row, col) = to_check.pop_front().unwrap();
        for (dr, dc) in NEIGHBORS {
            if (row == 0 && dr == -1)
                || (row == N_ROWS - 1 && dr == 1)
                || (col == 0 && dc == -1)
                || (col == N_COLS - 1 && dc == 1) {
                continue;
            }

            let nr = (row as i32 + dr) as usize;
            let nc = (col as i32 + dc) as usize;

            grid[nr][nc] += 1;

            if grid[nr][nc] > 9 && !has_flashed[nr][nc] {
                n_flashes += 1;
                has_flashed[nr][nc] = true;
                to_check.push_back((nr, nc));
            }
        }
    }

    for row in 0..N_ROWS {
        for col in 0..N_COLS {
            if grid[row][col] > 9 {
                grid[row][col] = 0;
            }
        }
    }

    n_flashes
}


fn part1(grid: &Input) -> i64 {
    let mut grid = grid.clone();
    (0..100).map(|_| do_step(&mut grid)).sum()
}

fn part2(grid: &Input) -> i64 {
    let mut grid = grid.clone();
    let mut step = 0;
    let mut n_flashes = 0;
    while n_flashes != 100 {
        n_flashes = do_step(&mut grid);
        step += 1;
    }

    step
}

pub fn day11() {
    let input = input::get_input(11);
    let grid = parse_input(&input);

    let t0 = Instant::now();
    println!("Part1: {}", part1(&grid));
    println!("Part2: {}", part2(&grid));
    println!("Time: {} us", t0.elapsed().as_micros());
}
