use std::fs;

fn parse_input(filename: &str) -> (u32, Vec<String>) {
    let mut data: Vec<(char, i32)> = Vec::new();
    let contents = fs::read_to_string(filename).expect("can't find file");
    let lines: Vec<&str> = contents.lines().collect();

    let timestamp: u32 = lines[0].parse().unwrap();

    let buses: Vec<String> = lines[1].split(",").map(|s| s.to_string()).collect();
    (timestamp, buses)
}

fn part1((timestamp, buses): &(u32, Vec<String>)) -> u32 {
    let mut min_waittime: u32 = 999999;
    let mut ans: u32 = 0;
    for bus in buses {
        match bus.as_str() {
            "x" => (),
            _ => {
                let freq: u32 = bus.parse().unwrap();
                let wait_time = (timestamp / freq + 1) * freq - timestamp;
                if wait_time < min_waittime {
                    min_waittime = wait_time;
                    ans = min_waittime * freq;
                }
            }
        }
    }
    ans
}

fn part2((_, buses): &(u32, Vec<String>)) -> u64 {
    let delay_and_bus: Vec<(u64, u64)> = buses
        .iter()
        .enumerate()
        .filter(|(i, s)| s.as_str() != "x")
        .map(|(i, s)| (i as u64, s.parse().unwrap()))
        .collect();
    // println!("{:?}", delay_and_bus);

    let mut offset: u64 = 1;
    let mut stride: u64 = 1;
    for (delay, bus) in delay_and_bus {
        let mut x: u64 = offset;
        while (x + delay) % bus != 0 {
            x += stride;
        }
        stride *= bus;
        offset = x;
        // println!("{}, {}", offset, stride);
    }
    offset
}
fn main() {
    let data = parse_input("input/input13.txt");

    let ans_part1 = part1(&data);
    println!("part1: {}", ans_part1);

    let ans_part2 = part2(&data);
    println!("part2: {}", ans_part2);
}
