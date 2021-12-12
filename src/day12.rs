use crate::input;
use std::time::Instant;
use std::collections::{HashMap,HashSet};
use itertools::Itertools;


type Input = HashMap<String, HashSet<String>>;

fn parse_input(input: &String) -> Input {
    let mut connections: Input = HashMap::new();
    input
        .trim()
        .lines()
        .map(|l| l.split("-").map(String::from).collect_tuple().unwrap())
        .for_each(|(k, v)| {
            connections.entry(k.clone()).or_insert(HashSet::new()).insert(v.clone());
            connections.entry(v.clone()).or_insert(HashSet::new()).insert(k.clone());
        });

    connections
}

fn is_ascii_lowercase(s: &str) -> bool {
    return s.is_ascii() && s.to_ascii_lowercase() == s;
}


fn find_paths(start_point: &String, connections: &Input, visited: &mut HashSet<String>) -> Vec<Vec<String>> {
    let mut paths = Vec::new();

    if is_ascii_lowercase(&start_point) {
        visited.insert(start_point.clone());
    }


    for cave in connections.get(start_point).unwrap() {
        if visited.contains(cave) {
            continue;
        } else if cave == "end" {
            paths.push(vec![start_point.clone(), String::from("end")]);
        } else {
            for p in find_paths(&cave, &connections, &mut visited.clone()) {
                let mut path = Vec::new();
                path.push(start_point.clone());
                path.extend(p);
                paths.push(path.clone());
            }
        }
    }
    paths
}


fn find_paths_2(start_point: &String, connections: &Input, visited: &mut HashSet<String>, small_seen_twice: bool) -> Vec<Vec<String>> {
    let mut paths = Vec::new();
    let mut small_seen_twice = small_seen_twice;

    if is_ascii_lowercase(&start_point) {
        if visited.contains(start_point) {
            small_seen_twice = true;
        } else {
            visited.insert(start_point.clone());
        }
    }


    for cave in connections.get(start_point).unwrap() {
        if (visited.contains(cave) && small_seen_twice) || cave == "start" {
            continue;
        } else if cave == "end" {
            paths.push(vec![start_point.clone(), String::from("end")]);
        } else {
            for p in find_paths_2(&cave, &connections, &mut visited.clone(), small_seen_twice) {
                let mut path = Vec::new();
                path.push(start_point.clone());
                path.extend(p);
                paths.push(path.clone());
            }
        }
    }
    paths
}



fn part1(connections: &Input) -> usize {
    find_paths(&String::from("start"), &connections, &mut HashSet::new()).len()
}

fn part2(connections: &Input) -> usize {
    find_paths_2(&String::from("start"), &connections, &mut HashSet::new(), false).len()
}

pub fn day12() {
    let input = input::get_input(12);
    let connections = parse_input(&input);

    let t0 = Instant::now();
    println!("Part1: {}", part1(&connections));
    println!("Part2: {}", part2(&connections));
    println!("Time: {} us", t0.elapsed().as_micros());
}
