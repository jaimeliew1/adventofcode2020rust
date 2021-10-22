use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

lazy_static! {
    static ref RE_MEM: Regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
    static ref RE_MASK: Regex = Regex::new(r"mask = ([X\d]+)").unwrap();
}

#[derive(Debug)]
enum Command {
    Mask(String),
    Mem { addr: u64, val: u64 },
}
//
// assert_eq!(format!("{:b}", 32), "101010");

//     let c = a & b;  // Bitwise AND, 0  (01 && 10 -> 00)
//    let d = a | b;  // Bitwise OR, 3  (01 || 10 -> 11)

// let bin_idx = "01110011001";
// let intval = isize::from_str_radix(bin_idx, 2).unwrap();

fn parse_input(filename: &str) -> Vec<Command> {
    let mut out: Vec<Command> = Vec::new();
    let contents = fs::read_to_string(filename).expect("can't find file");
    for line in contents.lines() {
        match RE_MEM.captures(line) {
            Some(cap) => out.push(Command::Mem {
                addr: cap[1].parse().unwrap(),
                val: cap[2].parse().unwrap(),
            }),
            None => (),
        }
        match RE_MASK.captures(line) {
            Some(cap) => out.push(Command::Mask(String::from(&cap[1]))),
            None => (),
        }
    }
    out
}

fn part1(commands: &Vec<Command>) -> u64 {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut ormask: u64 = 0;
    let mut andmask: u64 = 0;
    for command in commands {
        match command {
            Command::Mask(mask) => {
                ormask = isize::from_str_radix(mask.replace("X", "0").as_str(), 2).unwrap() as u64;
                andmask = isize::from_str_radix(mask.replace("X", "1").as_str(), 2).unwrap() as u64;
            }
            Command::Mem { addr, val } => {
                let to_enter = (val & andmask) | ormask;
                *memory.entry(*addr).or_insert(to_enter) = to_enter;
            }
        }
    }
    memory.values().fold(0, |acc, x| acc + x)
}

fn part2(commands: &Vec<Command>) -> u64 {
    let mut memory: HashMap<u64, u64> = HashMap::new();

    let mut protomask: u64 = 0;
    let mut floatmask: u64 = 0;
    let mut float_pos: Vec<usize> = Vec::new();
    for command in commands {
        match command {
            Command::Mask(mask) => {
                // masks.clear();
                protomask = isize::from_str_radix(&mask.replace("X", "0"), 2).unwrap() as u64;
                floatmask = isize::from_str_radix(&mask.replace("0", "1").replace("X", "0"), 2)
                    .unwrap() as u64;
                float_pos = mask
                    .chars()
                    .rev()
                    .collect::<String>()
                    .match_indices("X")
                    .map(|(i, _)| i)
                    .collect();
            }
            Command::Mem { addr, val } => {
                for blah in 0..2_u64.pow(float_pos.len() as u32) {
                    let mut thisaddr = (addr | protomask) & floatmask;
                    // add the float values to thisaddr.
                    for (i, pos) in float_pos.iter().enumerate() {
                        thisaddr += ((blah >> i) % 2) << pos;
                    }

                    *memory.entry(thisaddr).or_insert(*val) = *val;
                }
            }
        }
    }
    memory.values().fold(0, |acc, x| acc + x)
}

fn main() {
    let now = std::time::Instant::now();

    let data = parse_input("input/input14.txt");

    let ans_part1 = part1(&data);
    println!("part1: {}", ans_part1);

    let ans_part2 = part2(&data);
    println!("part2: {}", ans_part2);

    let time = now.elapsed().as_micros();
    println!("Time: {}ns", time);
}
