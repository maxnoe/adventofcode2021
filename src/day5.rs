use crate::input;
use std::collections::HashMap;
use std::time::Instant;
use std::cmp::{min, max};


#[derive(Debug)]
#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: u16,
    y: u16,
}



#[derive(Debug)]
struct Line {
    p0: Point,
    p1: Point,
}



impl Line {
    pub fn is_horizontal(&self) -> bool {
        self.p0.y == self.p1.y
    }

    pub fn is_vertical(&self) -> bool {
        self.p0.x == self.p1.x
    }

    pub fn is_axis_parallel(&self) -> bool {
        self.is_vertical() || self.is_horizontal()
    }

    pub fn is_diagonal(&self) -> bool {
        !self.is_axis_parallel()
    }
}


fn parse_point(point: &str) -> Point {
    let mut iter = point.split(",");
    let x: u16 = iter.next().unwrap().parse().unwrap();
    let y: u16 = iter.next().unwrap().parse().unwrap();

    Point{x, y}
}


fn parse_line(line: &str) -> Line {
    let points: Vec<Point> = line.split(" -> ")
        .map(parse_point)
        .collect();

    if points.len() != 2 {
        panic!("Did not find 2 lines");
    }

    Line{p0: points[0], p1: points[1]}
}


fn parse_input(input: &String) -> Vec<Line> {
    input
        .lines()
        .map(parse_line)
        .collect()
}


fn add_vents(vents: &mut HashMap<Point, u16>, line: &Line) {
    if line.is_horizontal() {
        let y = line.p0.y;
        let start = min(line.p0.x, line.p1.x);
        let end = max(line.p0.x, line.p1.x);

        for x in start..=end {
            let point = Point{x, y};
            match vents.get_mut(&point) {
                Some(val) => {*val += 1},
                None => {vents.insert(point, 1);},
            }
        }
    } else if line.is_vertical() {
        let x = line.p0.x;
        let start = min(line.p0.y, line.p1.y);
        let end = max(line.p0.y, line.p1.y);

        for y in start..=end {
            let point = Point{x, y};
            match vents.get_mut(&point) {
                Some(val) => {*val += 1},
                None => {vents.insert(point, 1);},
            }
        }
    }
}

fn add_vents_diagonal(vents: &mut HashMap<Point, u16>, line: &Line) {
    if line.is_diagonal() {
        let delta_x: i32 = line.p1.x as i32 - line.p0.x as i32;
        let delta_y: i32 = line.p1.y as i32 - line.p0.y as i32;
        let n = delta_x.abs();
        let delta_x = delta_x.signum();
        let delta_y = delta_y.signum();

        for i in 0..=n {
            let point = Point{
                x: (line.p0.x as i32 + i * delta_x) as u16,
                y: (line.p0.y as i32 + i * delta_y) as u16,
            };
            match vents.get_mut(&point) {
                Some(val) => {*val += 1},
                None => {vents.insert(point, 1);},
            }
        }
    }
}


fn count_at_least_2(vents: &HashMap<Point, u16>) -> u32 {
    vents.iter()
        .map(|(_, v)| *v)
        .filter(|v| *v >= 2)
        .count() as u32
}


fn part1(lines: &Vec<Line>) -> u32 {
    let mut vents: HashMap<Point, u16> = HashMap::new();

    lines.iter()
        .filter(|l| l.is_axis_parallel())
        .for_each(|l| add_vents(&mut vents, l));
    
    count_at_least_2(&vents)
}

fn part2(lines: &Vec<Line>) -> u32 {
    let mut vents: HashMap<Point, u16> = HashMap::new();

    lines.iter()
        .for_each(|l| {
            add_vents(&mut vents, l);
            add_vents_diagonal(&mut vents, l);
        });
    
    count_at_least_2(&vents)
}


pub fn day5() {
    let input = input::get_input(5);
    let lines = parse_input(&input);

    let t0 = Instant::now();
    println!("Part1: {}", part1(&lines));
    println!("Part2: {}", part2(&lines));
    println!("Time: {} us", t0.elapsed().as_micros());
}
