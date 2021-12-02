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
        _ => {
            println!("Day {} not yet implemented", day);
            std::process::exit(1);
        }
    }
}
