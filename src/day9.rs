use crate::input;
use std::time::Instant;
use std::collections::VecDeque;
use itertools::Itertools;


fn parse_input(input: &String) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c as u8 - b'0').collect())
        .collect()
}


const DIRECTIONS: [(i32, i32); 4] = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
];


fn is_min(grid: &Vec<Vec<u8>>, row: i32, col: i32) -> bool {
    let value = grid[row as usize][col as usize];
    let n_rows: i32 = grid.len().try_into().unwrap();
    let n_cols: i32 = grid[0].len().try_into().unwrap();

    DIRECTIONS
        .iter()
        .map(|(drow, dcol)| (row + drow, col + dcol))
        .filter(|(r, c)| (*r >= 0) && (*r < n_rows) && (*c >= 0) && (*c < n_cols))
        .all(|(r, c)| grid[r as usize][c as usize] > value)
}

fn local_minima(grid: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let mut minima = Vec::new();
    let n_rows = grid.len();
    let n_cols = grid[0].len();

    for row in 0..n_rows {
        for col in 0..n_cols {
            if is_min(grid, row as i32, col as i32) {
                minima.push((row, col));
            }
        }
    }

    minima
}


fn basin_size(row: usize, col: usize, grid: &Vec<Vec<u8>>, visited: &mut Vec<Vec<bool>>) -> usize {
    let mut size = 1;
    let mut to_check: VecDeque<(usize, usize)> = VecDeque::new();

    let n_rows: i32 = grid.len().try_into().unwrap();
    let n_cols: i32 = grid[0].len().try_into().unwrap();

    if visited[row][col] {
        return 0;
    }

    visited[row][col] = true;

    DIRECTIONS
        .iter()
        .map(|(drow, dcol)| (row as i32 + drow, col as i32 + dcol))
        .filter(|(r, c)| (*r >= 0) && (*r < n_rows) && (*c >= 0) && (*c < n_cols) && !visited[*r as usize][*c as usize])
        .for_each(|(r, c)| to_check.push_back((r as usize, c as usize)));


    while to_check.len() > 0 {
        let (r, c) = to_check.pop_front().unwrap();

        if grid[r][c] < 9 && !visited[r][c] {
            size += 1;

            DIRECTIONS
                .iter()
                .map(|(drow, dcol)| (r as i32 + drow, c as i32 + dcol))
                .filter(|(r, c)| (*r >= 0) && (*r < n_rows) && (*c >= 0) && (*c < n_cols) && !visited[*r as usize][*c as usize])
                .for_each(|(r, c)| to_check.push_back((r as usize, c as usize)));
        }

        visited[r][c] = true;

    }

    size
}

fn part1(grid: &Vec<Vec<u8>>) -> usize {
    local_minima(grid).iter().map(|(r, c)| grid[*r][*c] as usize + 1).sum()
}

fn part2(grid: &Vec<Vec<u8>>) -> usize {
    let n_rows = grid.len();
    let n_cols = grid[0].len();
    let mut visited: Vec<Vec<bool>> = vec![vec![false; n_cols]; n_rows];
    local_minima(grid)
        .iter()
        .map(|(r, c)| basin_size(*r, *c, grid, &mut visited))
        .sorted()
        .rev()
        .take(3)
        .product()
}

pub fn day9() {
    let input = input::get_input(9);
    let grid = parse_input(&input);

    let t0 = Instant::now();
    println!("Part1: {}", part1(&grid));
    println!("Part2: {}", part2(&grid));
    println!("Time: {} us", t0.elapsed().as_micros());
}
