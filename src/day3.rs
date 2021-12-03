use crate::input;

#[cfg(test)]
mod test {
    const TEST_INPUT: &str = "00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010";

    #[test]
    fn part1() {
        let (n_bits, numbers) = super::parse_input(&String::from(TEST_INPUT));
        assert_eq!(super::part1(n_bits, &numbers), 198)
    }

    #[test]
    fn oxygen() {
        let (n_bits, numbers) = super::parse_input(&String::from(TEST_INPUT));
        assert_eq!(super::find_ozygen_rating(n_bits, &numbers), 23)
    }
}

fn parse_input(input: &String) -> (usize, Vec<u16>) {
    let numbers = input
        .trim()
        .lines()
        .map(|line: &str| u16::from_str_radix(line.trim(), 2))
        .map(Result::unwrap)
        .collect();
    let n_bits = input.trim().lines().next().unwrap().trim().len();

    (n_bits, numbers)
}

fn most_common_bit(numbers: &Vec<u16>) -> [u16; 16] {
    let mut n_ones: [usize; 16] = [0; 16];
    let mut most_common: [u16; 16] = [0; 16];

    for number in numbers {
        for bit in 0..16 {
            if (number & (1 << bit)) > 0 {
                n_ones[bit] += 1;
            }
        }
    }

    for bit in 0..16 {
        let n_zeros = numbers.len() - n_ones[bit];
        if n_ones[bit] >= n_zeros {
            most_common[bit] = 1;
        }
    }

    most_common
}

fn part1(n_bits: usize, numbers: &Vec<u16>) -> u32 {
    let most_common = most_common_bit(&numbers);
    let mut gamma_rate: u16 = 0;
    for bit in 0..n_bits {
        gamma_rate += most_common[bit] << bit;
    }
    let epsilon_rate: u16 = (!gamma_rate) & ((1 << n_bits) - 1);

    gamma_rate as u32 * epsilon_rate as u32
}

fn find_ozygen_rating(n_bits: usize, numbers: &Vec<u16>) -> u16 {
    let mut numbers = numbers.clone();

    for bit in (0..n_bits).rev() {
        let most_common = most_common_bit(&numbers);

        numbers = numbers
            .iter()
            .filter(|number| (most_common[bit] << bit) == (*number & (1 << bit)))
            .copied()
            .collect();

        if numbers.len() == 1 {
            return numbers[0];
        }
    }

    panic!("Did not find rating");
}

fn find_carbon_rating(n_bits: usize, numbers: &Vec<u16>) -> u16 {
    let mut numbers = numbers.clone();

    for bit in (0..n_bits).rev() {
        let most_common = most_common_bit(&numbers);

        numbers = numbers
            .iter()
            .filter(|number| (most_common[bit] << bit) != (*number & (1 << bit)))
            .copied()
            .collect();

        if numbers.len() == 1 {
            return numbers[0];
        }
    }

    panic!("Did not find rating");
}

fn part2(n_bits: usize, numbers: &Vec<u16>) -> i64 {
    find_carbon_rating(n_bits, &numbers) as i64 * find_ozygen_rating(n_bits, &numbers) as i64
}

pub fn day3() {
    let input = input::get_input(3);
    let (n_bits, numbers) = parse_input(&input);
    println!("Part1: {}", part1(n_bits, &numbers));
    println!("Part2: {}", part2(n_bits, &numbers));
}
