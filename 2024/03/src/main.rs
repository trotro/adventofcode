use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn extract_numbers(input: &String) -> Vec<i32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let caps = re.captures_iter(input).map(|n| {
        let num1 = n.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let num2 = n.get(2).unwrap().as_str().parse::<i32>().unwrap();
        num1 * num2
    });
    return caps.collect();
}

fn main() {
    let mut sum: i32 = 0;
    if let Ok(instructions) = read_lines("./input") {
        // Consumes the iterator, returns an (Optional) String
        for line in instructions.flatten() {
            let extracted_numbers = extract_numbers(&line);
            sum += extracted_numbers.iter().sum::<i32>();
        }
    }
    println!("Result is {}", sum)
}
