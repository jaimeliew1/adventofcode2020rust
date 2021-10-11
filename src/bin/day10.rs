use std::fs;

fn parse_input(filename: &str) -> Vec<u64> {
    let raw = fs::read_to_string(filename).expect("can't find file");
    raw.lines().map(|s| s.parse::<u64>().unwrap()).collect()
}

fn part1(input: &Vec<u64>) -> u64 {
    let mut sorted_input = input.to_vec();
    sorted_input.sort();
    sorted_input.insert(0, 0);
    sorted_input.push(sorted_input.last().unwrap() + 3);
    let mut count1 = 0;
    let mut count3 = 0;

    for (i, a) in sorted_input.iter().enumerate() {
        if i == 0 {
            continue;
        }
        match a - sorted_input[i - 1] {
            1 => count1 += 1,
            3 => count3 += 1,
            _ => (),
        }
    }

    count1 * count3
}

fn part2(input: &Vec<u64>) -> u64 {
    let mut sorted_input = input.to_vec();
    sorted_input.sort();
    sorted_input.insert(0, 0);
    sorted_input.push(sorted_input.last().unwrap() + 3);

    let mut runs: Vec<u64> = Vec::new();

    let mut run_count = 0;
    for (i, a) in sorted_input.iter().enumerate() {
        if i == 0 {
            continue;
        }
        let diff = a - sorted_input[i - 1];
        match diff {
            1 => run_count += 1,
            3 => {
                if run_count > 0 {
                    runs.push(run_count);
                    run_count = 0;
                }
            }
            _ => (),
        }
    }
    // let trib = tribonacci(6);
    let trib = vec![1, 1, 2, 4, 7, 13, 24];
    let mut ans: u64 = 1;
    for run in runs {
        ans *= trib[run as usize];
    }
    ans
}
fn main() {
    let now = std::time::Instant::now();
    
    let input = parse_input("input/input10.txt");

    let ans_part1 = part1(&input);
    println!("part1: {}", ans_part1);

    let ans_part2 = part2(&input);
    println!("part2: {}", ans_part2);
    
    let time = now.elapsed().as_micros();
    println!("Time: {}ns", time);
}
