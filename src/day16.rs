use crate::input;
use std::time::Instant;
use phf::{Map, phf_map};


const HEX2BITS: Map<char, [u8; 4]> = phf_map! {
    '0' => [0, 0, 0, 0], '1' => [0, 0, 0, 1], '2' => [0, 0, 1, 0], '3' => [0, 0, 1, 1],
    '4' => [0, 1, 0, 0], '5' => [0, 1, 0, 1], '6' => [0, 1, 1, 0], '7' => [0, 1, 1, 1],
    '8' => [1, 0, 0, 0], '9' => [1, 0, 0, 1], 'A' => [1, 0, 1, 0], 'B' => [1, 0, 1, 1],
    'C' => [1, 1, 0, 0], 'D' => [1, 1, 0, 1], 'E' => [1, 1, 1, 0], 'F' => [1, 1, 1, 1],
};

struct Header {
    version: u8,
    type_id: u8,
}

struct Literal {
    header: Header,
    value: u64,
}

impl Literal {
    fn eval(&self) -> u64 {self.value}
}

struct Operator {
    header: Header,
    packets: Vec<Packet>
}

impl Operator {
    fn eval(&self) -> u64 {
        match self.header.type_id {
            0 => self.packets.iter().map(|p| p.eval()).sum(),
            1 => self.packets.iter().map(|p| p.eval()).product(),
            2 => self.packets.iter().map(|p| p.eval()).min().unwrap(),
            3 => self.packets.iter().map(|p| p.eval()).max().unwrap(),
            5 => (self.packets[0].eval() > self.packets[1].eval()) as u64,
            6 => (self.packets[0].eval() < self.packets[1].eval()) as u64,
            7 => (self.packets[0].eval() == self.packets[1].eval()) as u64,
            _ => panic!("Unexpected type {}", self.header.type_id)
        }
    }
}

enum Packet {
    Literal(Literal),
    Operator(Operator),
}

impl Packet {
    fn eval(&self) -> u64 {
        match self {
            Packet::Literal(lit) => lit.eval(),
            Packet::Operator(op) => op.eval(),
        }
    }
}

type BitStream = Vec<u8>;

fn parse_int(input: &BitStream, pos: &mut usize, bits: u8) -> u16 {
    let mut val: u16 = 0;
    for bit in (0..bits).rev() {
        val |= (input[*pos] as u16) << bit;
        *pos += 1;
    }
    val
}

fn parse_header(input: &BitStream, pos: &mut usize) -> Header {
    let version = parse_int(input, pos, 3) as u8;
    let type_id = parse_int(input, pos, 3) as u8;
    Header{version, type_id}
}

fn parse_input(input: &String) -> BitStream {
    input.trim().chars().map(|c| HEX2BITS[&c]).flatten().collect()
}

fn parse_literal(input: &BitStream, pos: &mut usize) -> u64 {
    let mut last = false;

    let mut parts = Vec::new();
    while !last {
        last = input[*pos] == 0;
        *pos += 1;
        parts.push(parse_int(input, pos, 4));
    }

    let val = parts.iter().rev().enumerate().map(|(n, v)| (*v as u64) << (4 * n)).sum();

    val
}

fn parse_packet(input: &BitStream, pos: &mut usize) -> Packet {
    let header = parse_header(input, pos);

    let packet = if header.type_id == 4 {
        let lit = Literal{header, value: parse_literal(input, pos)};
        Packet::Literal(lit)
    } else {

        let size_in_bits = input[*pos] == 0;
        *pos += 1;

        let mut packets = Vec::new();

        if size_in_bits {
            let size = parse_int(input, pos, 15) as usize;
            let end = *pos + size;

            while *pos < end {
                packets.push(parse_packet(input, pos));
            }
        } else {
            let size = parse_int(input, pos, 11) as usize;

            while packets.len() < size {
                packets.push(parse_packet(input, pos));
            }

        }

        Packet::Operator(Operator{header, packets})
    };

    packet
}

fn version_sum(packet: &Packet) -> u64 {
    match packet {
        Packet::Literal(lit) => lit.header.version as u64,
        Packet::Operator(op) => op.header.version as u64 + op.packets.iter().map(version_sum).sum::<u64>()
    }
}

fn part1(packet: &Packet) -> u64 {
    version_sum(packet)
}

fn part2(packet: &Packet) -> u64 {
    packet.eval()
}

pub fn day16() {
    let input = input::get_input(16);
    let bitstream = parse_input(&input);
    let packet = parse_packet(&bitstream, &mut 0);

    let t0 = Instant::now();
    println!("Part1: {}", part1(&packet));
    println!("Part2: {}", part2(&packet));
    println!("Time: {} us", t0.elapsed().as_micros());
}
