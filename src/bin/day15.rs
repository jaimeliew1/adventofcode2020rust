use std::collections::HashMap;

fn simulate(inp_vec: &Vec<u32>, n_max: usize) -> u32 {
    let mut last_seen = inp_vec
        .iter()
        .enumerate()
        .map(|(i, v)| (*v, i + 1))
        .collect::<HashMap<u32, usize>>();

    let mut num: u32 = *inp_vec.last().unwrap();

    for i in inp_vec.len()..n_max {
        if last_seen.contains_key(&num) {
            num = (i - last_seen.insert(num, i).unwrap()) as u32;
        } else {
            last_seen.insert(num, i);
            num = 0;
        }
    }
    num
}

fn main() {
    let now = std::time::Instant::now();
    let input: Vec<u32> = [2, 1, 10, 11, 0, 6].to_vec();

    let ans_part1 = simulate(&input, 2020);
    println!("part1: {}", ans_part1);

    let ans_part2 = simulate(&input, 30000000);
    println!("part2: {}", ans_part2);

    let time = now.elapsed().as_micros();
    println!("Time: {}ns", time);
}
