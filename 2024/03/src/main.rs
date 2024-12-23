use regex::Regex;
use std::fs::File;
use std::io::{self, Read};

fn read_lines(filename: &str) -> io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn part1(memory: &str) -> usize {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    return re
        .captures_iter(memory)
        .filter(|n| !n.get(1).is_none())
        .map(|n| n[1].parse::<usize>().unwrap() * n[2].parse::<usize>().unwrap())
        .sum();
}

fn part2(memory: &str) -> usize {
    let re: Regex = Regex::new(r"(?s)don't\(\).*?(?:do\(\)|$)|mul\((\d+),(\d+)\)").unwrap();
    return re
        .captures_iter(memory)
        .filter(|n| !n.get(1).is_none())
        .map(|n| n[1].parse::<usize>().unwrap() * n[2].parse::<usize>().unwrap())
        .sum();
}

fn main() {
    if let Ok(instructions) = read_lines("./input") {
        println!("Part 1: {}", part1(&instructions));
        println!("Part 2: {}", part2(&instructions));
    }
}
