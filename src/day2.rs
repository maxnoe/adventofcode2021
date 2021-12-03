use crate::input;

enum Direction {
    Up,
    Down,
    Forward,
}

struct Command {
    direction: Direction,
    amount: i32,
}

struct Position {
    depth: i32,
    distance: i32,
}

struct AimedPosition {
    depth: i32,
    distance: i32,
    aim: i32,
}

fn parse_line(line: &str) -> Result<Command, &str> {
    let mut split = line.split(" ");

    let direction = match split.next() {
        Some("up") => Direction::Up,
        Some("down") => Direction::Down,
        Some("forward") => Direction::Forward,
        _ => return Err("Unknown direction"),
    };
    let amount: i32 = match split.next() {
        Some(val) => val.parse().unwrap_or(0),
        _ => return Err("Unknown amount"),
    };

    Ok(Command { direction, amount })
}

fn parse_input(input: &String) -> Vec<Command> {
    input.lines().map(parse_line).map(Result::unwrap).collect()
}

fn update_position(position: &mut Position, command: &Command) {
    match command.direction {
        Direction::Up => position.depth -= command.amount,
        Direction::Down => position.depth += command.amount,
        Direction::Forward => position.distance += command.amount,
    }
}

fn update_aimed_position(position: &mut AimedPosition, command: &Command) {
    match command.direction {
        Direction::Up => position.aim -= command.amount,
        Direction::Down => position.aim += command.amount,
        Direction::Forward => {
            position.distance += command.amount;
            position.depth += position.aim * command.amount;
        }
    }
}

fn part1(commands: &Vec<Command>) -> i32 {
    let mut position = Position {
        depth: 0,
        distance: 0,
    };
    for command in commands {
        update_position(&mut position, &command);
    }
    position.depth * position.distance
}

fn part2(commands: &Vec<Command>) -> i32 {
    let mut position = AimedPosition {
        depth: 0,
        distance: 0,
        aim: 0,
    };
    for command in commands {
        update_aimed_position(&mut position, &command);
    }
    position.depth * position.distance
}

pub fn day2() {
    let input = input::get_input(2);
    let commands = parse_input(&input);

    println!("Part1 {}", part1(&commands));
    println!("Part2 {}", part2(&commands));
}
