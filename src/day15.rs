use crate::input;
use std::time::Instant;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

type Input = Vec<Vec<u8>>;
type Node = (usize, usize);

const NEIGHBORS: [(i8, i8); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];


fn parse_input(input: &String) -> Input {
    input
        .lines()
        .map(|l| l.chars().map(|c| c as u8 - b'0').collect())
        .collect()
}

// Taken and adapted from the rust doc binary_heap example

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u64,
    position: Node,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // switch operands to invert
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn tiled_risk(row: usize, col: usize, n_tiles: usize, risk: &Input) -> u64 {
    if n_tiles == 1 {
        return risk[row][col] as u64;
    }

    let n_rows = risk.len();
    let n_cols = risk[0].len();

    let row_tile = (row / n_rows) as u64;
    let col_tile = (col / n_cols) as u64;

    let base_risk = risk[row % n_rows][col % n_cols] as u64;
    let new_risk = base_risk + row_tile + col_tile;

    if new_risk > 9 {
        return new_risk % 10 + 1;
    }

    new_risk
}


fn dijkstra(risk: &Input, n_tiles: usize) -> u64 {
    let n_rows = risk.len() * n_tiles;
    let n_cols = risk[0].len() * n_tiles;

    let mut dist = vec![vec![u64::MAX; n_cols]; n_rows];

    let target = (n_rows - 1, n_cols - 1);

    let mut heap = BinaryHeap::new();

    // setup start point
    heap.push(State{cost: 0, position: (0, 0)});
    dist[0][0] = 0;

    while let Some(State{cost, position}) = heap.pop() {
        let (row, col) = position;

        if position == target {return cost; }


        if cost > dist[row][col] { continue; }

        for (drow, dcol) in NEIGHBORS {
            if (drow == -1 && row == 0) || (dcol == -1 && col == 0) || (drow == 1 && row == (n_rows - 1)) || (dcol == 1 && col == (n_cols - 1)) {
                continue;
            }

            let next_row = (row as i64 + drow as i64) as usize;
            let next_col = (col as i64 + dcol as i64) as usize;
            let next_cost = tiled_risk(next_row, next_col, n_tiles, risk);

            let next = State{
                cost: cost + next_cost,
                position: (next_row, next_col),
            };

            if next.cost < dist[next_row][next_col] {
                heap.push(next);
                dist[next_row][next_col] = next.cost;
            }
        }
    }

    0
}


fn part1(input: &Input) -> u64 {
    dijkstra(input, 1)
}

fn part2(input: &Input) -> u64 {
    dijkstra(input, 5)
}

pub fn day15() {
    let input = input::get_input(15);
    let risk = parse_input(&input);

    let t0 = Instant::now();
    println!("Part1: {}", part1(&risk));
    println!("Part2: {}", part2(&risk));
    println!("Time: {} us", t0.elapsed().as_micros());
}
