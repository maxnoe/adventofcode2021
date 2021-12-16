use crate::input;
use std::time::Instant;
use std::cmp::{Ordering,max};
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


fn dijkstra(risk: &Input, tile: usize) -> u64 {
    let n_rows_original = risk.len();
    let n_rows = n_rows_original * tile;
    let n_cols_original = risk[0].len();
    let n_cols = n_cols_original * tile;

    let mut dist = vec![vec![u64::MAX; n_cols]; n_rows];

    let target = (n_rows - 1, n_cols - 1);

    let mut heap = BinaryHeap::new();

    // setup start point
    heap.push(State{cost: 0, position: (0, 0)});
    dist[0][0] = 0;

    while let Some(State{cost, position}) = heap.pop() {
        if position == target {return cost; }

        let (row, col) = position;

        if cost > dist[row][col] { continue; }

        for (drow, dcol) in NEIGHBORS {
            if (drow == -1 && row == 0) || (dcol == -1 && col == 0) || (drow == 1 && row == (n_rows - 1)) || (dcol == 1 && col == (n_cols - 1)) {
                continue;
            }

            let neighbor_row = (row as i64 + drow as i64) as usize;
            let neighbor_col = (col as i64 + dcol as i64) as usize;

            let row_tile = (neighbor_row / n_rows_original) as u64;
            let col_tile = (neighbor_col / n_cols_original) as u64;

            let original_cost = risk[neighbor_col % n_rows_original][neighbor_col % n_cols_original];
            let neighbor_cost = max((original_cost as u64 + row_tile + col_tile) % 10, 1);

            let next = State{
                cost: cost + neighbor_cost,
                position: (neighbor_row, neighbor_col),
            };

            if next.cost < dist[neighbor_row][neighbor_col] {
                heap.push(next);
                dist[neighbor_row][neighbor_col] = next.cost;
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
    let risk_levels = parse_input(&input);
    println!("Grid size: {}x{}", risk_levels.len(), risk_levels[0].len());

    let t0 = Instant::now();
    println!("Part1: {}", part1(&risk_levels));
    println!("Part2: {}", part2(&risk_levels));
    println!("Time: {} us", t0.elapsed().as_micros());
}
