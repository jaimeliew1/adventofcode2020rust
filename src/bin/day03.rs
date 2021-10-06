use std::fs;

fn trees_hit(down: u32, across: u32, datavec: &Vec<Vec<char>>) -> u32 {
    let mut count: u32 = 0;
    let n = datavec.len();
    let m = datavec[0].len() as u32;

    let n_max = n as u32 / down;
    for i in 0..n_max {
        if datavec[(i * down) as usize][(i * across % m) as usize] == '#' {
            count += 1;
        }
    }
    count
}

fn part1(datavec: &Vec<Vec<char>>) -> u32 {
    trees_hit(1, 3, &datavec)
}

fn part2(datavec: &Vec<Vec<char>>) -> u32 {
    let a = trees_hit(1, 1, &datavec);
    let b = trees_hit(1, 3, &datavec);
    let c = trees_hit(1, 5, &datavec);
    let d = trees_hit(1, 7, &datavec);
    let e = trees_hit(2, 1, &datavec);
    a * b * c * d * e
}
fn main() {
    let contents = fs::read_to_string("input/input03.txt").expect("can't find file");
    let datavec: Vec<Vec<char>> = contents.lines().map(|s| s.chars().collect()).collect();

    let ans_part1 = part1(&datavec);
    println!("part1: {}", ans_part1);

    let ans_part2 = part2(&datavec);
    println!("part2: {}", ans_part2);
}
