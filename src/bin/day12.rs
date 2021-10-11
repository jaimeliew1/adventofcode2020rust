use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

lazy_static! {
    static ref DIRS: HashMap<char, (i32, i32)> =
        [('N', (0, 1)), ('E', (1, 0)), ('S', (0, -1)), ('W', (-1, 0))]
            .iter()
            .cloned()
            .collect();
}

fn parse_input(filename: &str) -> Vec<(char, i32)> {
    let mut data: Vec<(char, i32)> = Vec::new();
    let contents = fs::read_to_string(filename).expect("can't find file");
    let re = Regex::new(r"(\w)(\d+)").unwrap();

    for cap in re.captures_iter(contents.as_str()) {
        let letter: char = *&cap[1].chars().nth(0).unwrap();
        let val: i32 = *&cap[2].parse::<i32>().unwrap();
        data.push((letter, val));
    }

    data
}

fn part1(data: &Vec<(char, i32)>) -> i32 {
    let mut dir: i32 = 1; // NESW
    let mut pos: (i32, i32) = (0, 0);
    for (command, value) in data {
        match command {
            'N' | 'E' | 'S' | 'W' => {
                let (dx, dy) = DIRS.get(command).unwrap();
                pos.0 += dx * value;
                pos.1 += dy * value;
            }
            'L' => dir = (dir - value / 90).rem_euclid(4),
            'R' => dir = (dir + value / 90).rem_euclid(4),
            'F' => {
                let (dx, dy) = DIRS.get(&['N', 'E', 'S', 'W'][dir as usize]).unwrap();
                pos.0 += dx * value;
                pos.1 += dy * value;
            }
            _ => (),
        }
    }
    pos.0.abs() + pos.1.abs()
}

fn turn_clockwise((x, y): (i32, i32), times: usize) -> (i32, i32) {
    let (mut x, mut y) = (x, y);
    for i in 0..times {
        let (xold, yold) = (x, y);
        x = yold;
        y = -xold;
    }
    (x, y)
}

fn part2(data: &Vec<(char, i32)>) -> i32 {
    let mut shippos: (i32, i32) = (0, 0);
    let mut waypoint: (i32, i32) = (10, 1);
    for (command, value) in data {
        match command {
            'N' | 'E' | 'S' | 'W' => {
                let (dx, dy) = DIRS.get(command).unwrap();
                waypoint.0 += dx * value;
                waypoint.1 += dy * value;
            }
            'L' => {
                let times = ((360 - value) / 90).rem_euclid(4);
                waypoint = turn_clockwise(waypoint, times as usize);
            }
            'R' => {
                let times = (value / 90).rem_euclid(4);
                waypoint = turn_clockwise(waypoint, times as usize);
            }
            'F' => {
                shippos.0 += waypoint.0 * value;
                shippos.1 += waypoint.1 * value;
            }
            _ => println!("{}, {:?}", command, value),
        }

    }
    
    shippos.0.abs() + shippos.1.abs()
}

fn main() {
    let now = std::time::Instant::now();
    
    let data = parse_input("input/input12.txt");

    let ans_part1 = part1(&data);
    println!("part1: {}", ans_part1);
    
    let ans_part2 = part2(&data);
    println!("part2: {}", ans_part2);
    
    let time = now.elapsed().as_micros();
    println!("Time: {}ns", time);
}
