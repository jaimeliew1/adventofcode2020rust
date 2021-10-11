use itertools::Itertools;
use std::fs;

fn parse_input(filename: &str) -> Vec<u64> {
    let raw = fs::read_to_string(filename).expect("can't find file");
    raw.lines().map(|s| s.parse::<u64>().unwrap()).collect()
}

fn part1(input: &Vec<u64>) -> u64 {
    for i in 25..input.len() {
        let preamble = &input[i - 25..i];
        let ways = preamble
            .iter()
            .tuple_combinations()
            .filter(|&(a, b)| a + b == input[i])
            .count();
        if ways == 0 {
            return input[i];
        }
    }
    0
}

fn part2(input: &Vec<u64>, target: u64) -> u64 {
    let input: Vec<u64> = input.iter().filter(|&x| x < &target).cloned().collect();
    let mut lower: usize = 0;
    let mut upper: usize = 1;
    while lower < input.len() {
        let segment = &input[lower..=upper];
        let sum: u64 = segment.iter().sum();
        if sum == target {
            return segment.iter().max().unwrap() + segment.iter().min().unwrap();
        } else if sum > target {
            lower += 1;
            upper = lower + 1;
        } else {
            upper += 1;
        }
    }
    0
}
fn main() {
    let now = std::time::Instant::now();
    
    let input = parse_input("input/input09.txt");

    let ans_part1 = part1(&input);
    println!("part1: {}", ans_part1);

    let ans_part2 = part2(&input, ans_part1);
    println!("part2: {}", ans_part2);
    
    let time = now.elapsed().as_micros();
    println!("Time: {}ns", time);
}
