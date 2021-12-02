use crate::input;

enum Direction {
    Up,
    Down,
    Forward,
}

struct Command {
    direction: Direction,
    amount: i32
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


fn parse_input(input: &String) -> Vec<Command> {
    let mut commands = Vec::new();

    for line in input.trim().split("\n") {
        let mut split = line.split(" ");
        let direction = match split.next().expect("Error parsing") {
            "up" => Direction::Up,
            "down" => Direction::Down,
            "forward" => Direction::Forward,
            _ => panic!("Unexpected direction")
        };
        let amount: i32 = split.next().expect("Error parsing").parse().expect("Error parsing");

        commands.push(Command{direction, amount});
    }

    commands
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
    let mut position = Position{depth: 0, distance: 0};
    for command in commands {
        update_position(&mut position, &command);
    }
    position.depth * position.distance
}


fn part2(commands: &Vec<Command>) -> i32 {
    let mut position = AimedPosition{depth: 0, distance: 0, aim: 0};
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
