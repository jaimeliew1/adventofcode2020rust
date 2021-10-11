use std::fs;
static DIRS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum State {
    Floor,
    Free,
    Occ,
}
fn parse_input(filename: &str) -> [[State; 91]; 92] {
    let raw = fs::read_to_string(filename).expect("can't find file");
    let mut out: [[State; 91]; 92] = [[State::Floor; 91]; 92];
    for (i, line) in raw.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                '.' => out[i][j] = State::Floor,
                'L' => out[i][j] = State::Free,
                '#' => out[i][j] = State::Occ,
                _ => (),
            }
        }
    }
    out
}

fn part1(layout: &[[State; 91]; 92]) -> u32 {
    let mut layout = layout.clone();
    let mut count: u32 = 0;
    loop {
        layout = iterate1(layout);
        let this_count = layout
            .iter()
            .map(|r| r.iter().filter(|&&s| s == State::Occ).count())
            .fold(0, |acc, x| acc + x) as u32;
        if count == this_count {
            break;
        } else {
            count = this_count
        }
    }
    count
}

fn iterate1(input: [[State; 91]; 92]) -> [[State; 91]; 92] {
    let mut output: [[State; 91]; 92] = [[State::Floor; 91]; 92];

    for (i, row) in input.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            let n = count_neighbours1(i, j, &input);
            output[i][j] = match c {
                State::Free if n == 0 => State::Occ,
                State::Occ if n >= 4 => State::Free,
                c => *c,
            };
        }
    }
    output
}

fn count_neighbours1(i: usize, j: usize, layout: &[[State; 91]; 92]) -> u32 {
    let dx: [i32; 3] = [-1, 0, 1];
    let mut count = 0;
    for x in dx.iter().map(|x| x + i as i32) {
        if let Some(row) = layout.get(x as usize) {
            count += dx
                .iter()
                .map(|y| y + j as i32)
                .filter_map(|y| row.get(y as usize))
                .filter(|&&val| val == State::Occ)
                .count() as u32;
        }
    }
    if layout[i][j] == State::Occ {
        count -= 1;
    }
    count
}

fn part2(layout: &[[State; 91]; 92]) -> u32 {
    let mut layout = layout.clone();
    let mut count: u32 = 0;
    loop {
        layout = iterate2(layout);
        let this_count = layout
            .iter()
            .map(|r| r.iter().filter(|&&s| s == State::Occ).count())
            .fold(0, |acc, x| acc + x) as u32;
        if count == this_count {
            break;
        } else {
            count = this_count
        }
    }
    count
}

fn iterate2(input: [[State; 91]; 92]) -> [[State; 91]; 92] {
    let mut output: [[State; 91]; 92] = [[State::Floor; 91]; 92];

    for (i, row) in input.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            let n = DIRS
                .iter()
                .filter(|dir| neighbours_in_dir(**dir, i, j, &input))
                .count() as u32;
                
            output[i][j] = match c {
                State::Free if n == 0 => State::Occ,
                State::Occ if n >= 5 => State::Free,
                c => *c,
            };
        }
    }
    output
}

fn neighbours_in_dir((dx, dy): (i32, i32), i: usize, j: usize, layout: &[[State; 91]; 92]) -> bool {
    let i = (i as i32 + dx) as usize;
    let j = (j as i32 + dy) as usize;

    match layout.get(i).and_then(|row| row.get(j)) {
        Some(State::Occ) => return true,
        Some(State::Free) => return false,
        None => return false,
        _ => return neighbours_in_dir((dx, dy), i, j, &layout),
    };
}

fn main() {
    let now = std::time::Instant::now();
        
    let layout = parse_input("input/input11.txt");

    let ans_part1 = part1(&layout);
    println!("part1: {}", ans_part1);

    let ans_part2 = part2(&layout);
    println!("part2: {}", ans_part2);
    
    let time = now.elapsed().as_micros();
    println!("Time: {}ns", time);
}
