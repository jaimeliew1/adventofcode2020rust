use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn parse_input(filename: &str) -> HashMap<String, Vec<(String, u32)>> {
    let bag_content_parser = Regex::new(r"(\d+) ([\w\s]+) bags?").unwrap();
    let raw = fs::read_to_string(filename).expect("can't find file");
    let lines: Vec<&str> = raw.lines().collect();

    let mut bagdict: HashMap<String, Vec<(String, u32)>> = HashMap::new();

    for line in lines {
        let (bag, contents) = line.split_once(" bags contain").unwrap();

        let a: Vec<(String, u32)> = bag_content_parser
            .captures_iter(contents)
            .map(|cap| (String::from(&cap[2]), *&cap[1].parse::<u32>().unwrap()))
            .collect();

        bagdict.insert(String::from(bag), a);
    }
    bagdict
}

fn contains_shinygold(bag: &String, bagdict: &HashMap<String, Vec<(String, u32)>>) -> bool {
    let thesebags = bagdict.get(bag).unwrap();
    if thesebags.len() == 0 {
        return false;
    }
    if thesebags.iter().any(|x| x.0.contains("shiny gold")) {
        return true;
    }
    return thesebags.iter().any(|x| contains_shinygold(&x.0, &bagdict));
}
fn count_bags(bag: &String, bagdict: &HashMap<String, Vec<(String, u32)>>) -> u32 {
    bagdict
        .get(bag)
        .unwrap()
        .iter()
        .fold(0, |acc, (bagname, n)| {
            acc + n + n * count_bags(bagname, &bagdict)
        })
}

fn part1(bagdict: &HashMap<String, Vec<(String, u32)>>) -> u32 {
    bagdict
        .keys()
        .filter(|bag| contains_shinygold(bag, &bagdict))
        .count() as u32
}

fn part2(bagdict: &HashMap<String, Vec<(String, u32)>>) -> u32 {
    count_bags(&String::from("shiny gold"), &bagdict)
}
fn main() {
    let bagdict = parse_input("input/input07.txt");

    let ans_part1 = part1(&bagdict);
    println!("part1: {}", ans_part1);

    let ans_part2 = part2(&bagdict);
    println!("part2: {}", ans_part2);
}
