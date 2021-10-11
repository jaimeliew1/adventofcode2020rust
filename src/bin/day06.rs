use std::collections::HashSet;
use std::fs;

fn part1(chunks: &Vec<&str>) -> u32 {
    let mut total_count: u32 = 0;
    // Put all answers in single line
    let chunks: Vec<String> = chunks.iter().map(|s| s.replace("\n", "")).collect();
    for char_vec in chunks {
        let mut letter_set: HashSet<char> = HashSet::new();
        for c in char_vec.chars() {
            letter_set.insert(c);
        }
        total_count += letter_set.len() as u32;
    }
    total_count
}

fn part2(chunks: &Vec<&str>) -> u32 {
    let mut total_count: u32 = 0;
    for char_vec in chunks {
        // Get unique characters
        let mut letter_set: HashSet<char> = HashSet::new();
        for c in char_vec.chars() {
            if c != '\n' {
                letter_set.insert(c);
            }
        }

        for c in letter_set {
            if char_vec.split('\n').all(|s| s.contains(c)) {
                total_count += 1;
            }
        }
    }
    total_count
}
fn main() {
    let now = std::time::Instant::now();
    
    let contents = fs::read_to_string("input/input06.txt").expect("can't find file");
    let chunks: Vec<&str> = contents.split("\n\n").collect();

    let ans_part1 = part1(&chunks);
    println!("part1: {}", ans_part1);

    let ans_part2 = part2(&chunks);
    println!("part2: {}", ans_part2);
    
    let time = now.elapsed().as_micros();
    println!("Time: {}ns", time);
}
