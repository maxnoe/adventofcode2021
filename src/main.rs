use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: aocmaxnoe2021 <day>");
        std::process::exit(1);
    }

    let day: u8 = args[1].parse().expect("Day must be a number");

    match day {
        1 => aocmaxnoe2021::day1(),
        2 => aocmaxnoe2021::day2(),
        3 => aocmaxnoe2021::day3(),
        4 => aocmaxnoe2021::day4(),
        5 => aocmaxnoe2021::day5(),
        6 => aocmaxnoe2021::day6(),
        7 => aocmaxnoe2021::day7(),
        8 => aocmaxnoe2021::day8(),
        9 => aocmaxnoe2021::day9(),
        10 => aocmaxnoe2021::day10(),
        11 => aocmaxnoe2021::day11(),
        12 => aocmaxnoe2021::day12(),
        13 => aocmaxnoe2021::day13(),
        14 => aocmaxnoe2021::day14(),
        15 => aocmaxnoe2021::day15(),
        16 => aocmaxnoe2021::day16(),
        17 => aocmaxnoe2021::day17(),
        18 => aocmaxnoe2021::day18(),
        _ => {
            println!("Day {} not yet implemented", day);
            std::process::exit(1);
        }
    }
}
