use regex::Regex;
use std::collections::HashSet;
use std::fs;

#[derive(Debug, Clone)]
enum Op {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

fn parse_input(filename: &str) -> Vec<Op> {
    let raw = fs::read_to_string(filename).expect("can't find file");
    let lines: Vec<&str> = raw.lines().collect();

    let mut out: Vec<Op> = Vec::new();
    let parser = Regex::new(r"(\w{3}) ([\+-]\d+)").unwrap();

    for line in lines {
        let cap = parser.captures(line).unwrap();
        let op = &cap[1];
        let val = *&cap[2].parse::<i32>().unwrap();
        match op {
            "nop" => out.push(Op::Nop(val)),
            "acc" => out.push(Op::Acc(val)),
            "jmp" => out.push(Op::Jmp(val)),
            _ => panic!("unknown command"),
        };
    }

    out
}

fn part1(commands: &Vec<Op>) -> i32 {
    let mut i: i32 = 0;
    let mut accumulator: i32 = 0;
    let mut visited: HashSet<i32> = HashSet::new();

    while !visited.contains(&i) {
        visited.insert(i);

        match commands[i as usize] {
            Op::Acc(x) => {
                accumulator += x;
                i += 1;
            }
            Op::Nop(_) => i += 1,
            Op::Jmp(x) => i += x,
        }
    }
    accumulator
}

fn run(commands: &Vec<Op>) -> Option<i32> {
    let mut i: i32 = 0;
    let mut accumulator: i32 = 0;
    let mut visited: HashSet<i32> = HashSet::new();

    loop {
        if visited.contains(&i) {
            return None;
        } else if i == commands.len() as i32 {
            return Some(accumulator);
        }
        visited.insert(i);

        match commands[i as usize] {
            Op::Acc(x) => {
                accumulator += x;
                i += 1;
            }
            Op::Nop(_) => i += 1,
            Op::Jmp(x) => i += x,
        }
    }
}

fn part2(commands: &Vec<Op>) -> Option<i32> {
    for (i, op) in commands.iter().enumerate() {
        let mut variation = commands.to_vec();
        variation[i] = match op {
            Op::Acc(x) => Op::Acc(*x),
            Op::Nop(x) => Op::Jmp(*x),
            Op::Jmp(x) => Op::Nop(*x),
        };
        if let Some(ans) = run(&variation) {
            return Some(ans);
        }
    }
    None
}
fn main() {
    let now = std::time::Instant::now();
    
    let commands = parse_input("input/input08.txt");

    let ans_part1 = part1(&commands);
    println!("part1: {}", ans_part1);

    let ans_part2 = part2(&commands).unwrap();
    println!("part2: {}", ans_part2);
    
    let time = now.elapsed().as_micros();
    println!("Time: {}ns", time);
}
