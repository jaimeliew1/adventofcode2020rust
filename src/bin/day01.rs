use std::fs;

fn part1(inp_vec: &Vec<i32>) -> i32 {
    for (i, a) in inp_vec.iter().enumerate() {
        for b in &inp_vec[i + 1..] {
            if a + b == 2020 {
                return a * b;
            }
        }
    }
    unreachable!()
}

fn part2(inp_vec: &Vec<i32>) -> i32 {
    for (i, a) in inp_vec.iter().enumerate() {
        for (j, b) in inp_vec[i + 1..].iter().enumerate() {
            for c in &inp_vec[j + 1..] {
                if a + b + c == 2020 {
                    return a * b * c;
                }
            }
        }
    }
    unreachable!()
}
fn main() {
    let now = std::time::Instant::now();
    
    let contents = fs::read_to_string("input/input01.txt").expect("can't find file");
    let inp_vec: Vec<i32> = contents
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();

    let ans_part1 = part1(&inp_vec);
    println!("part1: {}", ans_part1);

    let ans_part2 = part2(&inp_vec);
    println!("part2: {}", ans_part2);
    
    let time = now.elapsed().as_micros();
    println!("Time: {}ns", time);
}
