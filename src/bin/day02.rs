use regex::Regex;
use std::fs;

#[derive(Debug)]
struct Data {
    n1: u32,
    n2: u32,
    c: char,
    seq: String,
}

fn parse_line(line: &str, re: &Regex) -> Data {
    let cap = re.captures(line).unwrap();
    let data = Data {
        n1: cap.get(1).unwrap().as_str().parse::<u32>().unwrap(),
        n2: cap.get(2).unwrap().as_str().parse::<u32>().unwrap(),
        c: cap.get(3).unwrap().as_str().parse::<char>().unwrap(),
        seq: String::from(cap.get(4).unwrap().as_str()),
    };

    data
}

fn part1(datavec: &Vec<Data>) -> u32 {
    let mut ans: u32 = 0;
    for data in datavec.iter() {
        let count: u32 = data.seq.matches(data.c).count() as u32;
        if data.n1 <= count && data.n2 >= count {
            ans += 1;
        }
    }
    ans
}

fn part2(datavec: &Vec<Data>) -> u32 {
    let mut ans: u32 = 0;
    for data in datavec.iter() {
        let c1 = data.seq.chars().nth((data.n1 - 1) as usize).unwrap();
        let c2 = data.seq.chars().nth((data.n2 - 1) as usize).unwrap();

        if (c1 == data.c || c2 == data.c) && c1 != c2 {
            ans += 1;
        }
    }
    ans
}

fn main() {
    let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    let contents = fs::read_to_string("input/input02.txt").expect("can't find file");
    let datavec: Vec<Data> = contents.lines().map(|s| parse_line(s, &re)).collect();

    let ans_part1 = part1(&datavec);
    println!("part1: {}", ans_part1);

    let ans_part2 = part2(&datavec);
    println!("part2: {}", ans_part2);
}
