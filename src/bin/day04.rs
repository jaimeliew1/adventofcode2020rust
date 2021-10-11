use regex::Regex;
use std::fs;

fn part1(chunks: &Vec<String>) -> u32 {
    let mut count: u32 = 0;
    let attributes = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    for chunk in chunks {
        if attributes.iter().all(|s| chunk.contains(s)) {
            count += 1;
        }
    }
    count
}

fn part2(chunks: &Vec<String>) -> u32 {
    let mut count: u32 = 0;
    let identifiers = [
        Regex::new(r"byr:(19[2-9][0-9]|200[0-2])\s").unwrap(),
        Regex::new(r"iyr:(201[0-9]|2020)\s").unwrap(),
        Regex::new(r"eyr:(202[0-9]|2030)\s").unwrap(),
        Regex::new(r"hgt:(1[5-8][0-9]cm|19[0-3]cm|59in|6[0-9]in|7[0-6]in)\s").unwrap(),
        Regex::new(r"hcl:(#[0-9a-f]{6})\s").unwrap(),
        Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)\s").unwrap(),
        Regex::new(r"pid:[0-9]{9}\s").unwrap(),
    ];
    for chunk in chunks {
        if identifiers.iter().all(|id| id.is_match(chunk)) {
            count += 1;
        }
    }
    count
}

fn main() {
    let now = std::time::Instant::now();
    
    let contents = fs::read_to_string("input/input04.txt").expect("can't find file");
    // Add a space at the end to make the regex work.
    let chunks: Vec<String> = contents.split("\n\n").map(|s| format!("{} ", s)).collect();

    let ans_part1 = part1(&chunks);
    println!("part1: {}", ans_part1);

    let ans_part2 = part2(&chunks);
    println!("part2: {}", ans_part2);
    
    let time = now.elapsed().as_micros();
    println!("Time: {}ns", time);
}
