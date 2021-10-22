use std::collections::HashMap;

fn simulate(inp_vec: &Vec<u32>, n_max: u32) -> u32 {
    let mut i = 1;
    let mut num: u32;
    let mut last_seen: HashMap<u32, u32> = HashMap::new();

    for val in &inp_vec[0..inp_vec.len() - 1] {
        last_seen.insert(*val, i);
        i += 1;
    }

    i += 1;
    num = *inp_vec.last().unwrap();

    while i <= n_max {
        if last_seen.contains_key(&num) {
            let last_i = last_seen[&num];
            last_seen.insert(num, i - 1);
            num = i - 1 - last_i;
        } else {
            last_seen.insert(num, i - 1);
            num = 0;
        }
        i += 1;
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
