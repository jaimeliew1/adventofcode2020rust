use std::fs;

fn list_ids(lines: &Vec<Vec<char>>) -> Vec<u32> {
    let mut out = Vec::<u32>::new();
    for line in lines {
        let mut row: u32 = 0;

        for (i, c) in line.iter().rev().enumerate() {
            match c {
                'B' | 'R' => row += 2_i32.pow(i as u32) as u32,
                _ => (),
            }
        }

        out.push(row)
    }
    out
}

fn part1(lines: &Vec<Vec<char>>) -> u32 {
    *list_ids(&lines).iter().max().unwrap()
}

fn part2(lines: &Vec<Vec<char>>) -> u32 {
    let mut ids = list_ids(&lines);
    ids.sort();
    for i in 0..ids.len() {
        if ids[i] + 1 != ids[i + 1] {
            return (ids[i] + 1) as u32;
        }
    }
    unreachable!()
}

fn main() {
    let now = std::time::Instant::now();
    
    let contents = fs::read_to_string("input/input05.txt").expect("can't find file");
    let lines: Vec<Vec<char>> = contents.lines().map(|s| s.chars().collect()).collect();

    let ans_part1 = part1(&lines);
    println!("part1: {}", ans_part1);

    let ans_part2 = part2(&lines);
    println!("part2: {}", ans_part2);
    
    let time = now.elapsed().as_micros();
    println!("Time: {}ns", time);
}
