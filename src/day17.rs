use crate::input;
use std::time::Instant;
use lazy_static::lazy_static;
use regex::Regex;
use itertools::Itertools;
use std::cmp::Ordering;

#[derive(Debug)]
struct TargetArea {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32
}

impl TargetArea {
    fn is_inside(&self, x: i32, y: i32) -> bool {
        (x >= self.min_x)
        && (x <= self.max_x)
        && (y >= self.min_y)
        && (y <= self.max_y)
    }


    fn hits(&self, vx: i32, vy: i32) -> bool {
        let mut x = 0;
        let mut y = 0;
        let mut vx = vx;
        let mut vy = vy;

        while x <= self.max_x && y >= self.min_y {
            if self.is_inside(x, y) {
                return true;
            } 

            x += vx;
            y += vy;
            vx += match vx.cmp(&0) {
                Ordering::Less => 1,
                Ordering::Equal => 0,
                Ordering::Greater => -1,
            };
            vy -= 1;
        }

        false
    }

    fn possible_shots(&self) -> (i32, usize) {
        let mut best_height = 0;
        
        let mut n_shots = 0;

        for vx in 1..=self.max_x {
            for vy in self.min_y..self.min_y.abs() {
                let height = max_height(vy);
                if self.hits(vx, vy) {
                    n_shots += 1;

                    if height > best_height {best_height = height}
                }
                
            }
        }

        (best_height, n_shots)
    }

}

fn parse_input(input: &String) -> TargetArea {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)").unwrap();
    }

    let groups = RE.captures(input)
        .unwrap()
        .iter()
        .skip(1)
        .map(|m| m.unwrap().as_str())
        .map(str::parse::<i32>)
        .map(Result::unwrap)
        .collect_vec();

    TargetArea{
        min_x: groups[0],
        max_x: groups[1],
        min_y: groups[2],
        max_y: groups[3],
    }
}

fn max_height(vy: i32) -> i32 {
    if vy < 0 {
        0
    } else {
        vy * (vy + 1) / 2
    }
}


pub fn day17() {
    let input = input::get_input(17);
    let target_area = parse_input(&input);

    let t0 = Instant::now();
    let (part1, part2) = target_area.possible_shots();
    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
    println!("Time: {} us", t0.elapsed().as_micros());
}
