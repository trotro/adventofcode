use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn is_safe(levels: Vec<i32>) -> bool {
    let mut safe: bool = false;
    let mut increase: bool = false;
    let mut decrease: bool = false;
    for i in 1..levels.len() {
        if levels[i] != levels[i - 1] {
            for i in 1..levels.len() {
                match levels[i] {
                    val if val == levels[i - 1] + 1 => {
                        increase = true;
                        safe = true;
                    }
                    val if val == levels[i - 1] + 2 => {
                        increase = true;
                        safe = true;
                    }
                    val if val == levels[i - 1] + 3 => {
                        increase = true;
                        safe = true;
                    }
                    val if val == levels[i - 1] - 1 => {
                        decrease = true;
                        safe = true;
                    }
                    val if val == levels[i - 1] - 2 => {
                        decrease = true;
                        safe = true;
                    }
                    val if val == levels[i - 1] - 3 => {
                        decrease = true;
                        safe = true;
                    }
                    _ => {
                        return false;
                    }
                }
                if increase && decrease {
                    return false;
                }
            }
        } else {
            return false;
        }
    }
    return safe;
}

fn main() {
    let mut safe: i32 = 0;
    if let Ok(reports) = read_lines("./input") {
        // Consumes the iterator, returns an (Optional) String
        for report in reports.flatten() {
            let levels: Vec<i32> = report
                .split_whitespace()
                .map(|l| l.parse::<i32>().unwrap())
                .collect();
            if is_safe(levels) {
                safe += 1;
            }
        }
    }
    println!("Safe reports: {}", safe);
}
