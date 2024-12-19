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

fn is_safe(report: &Vec<i32>) -> bool {
    if report.is_sorted_by(|a, b| a < b && b - a < 4) {
        return true;
    }
    if report.is_sorted_by(|a, b| a > b && a - b < 4) {
        return true;
    }
    return false;
}

fn problem_dampener(mut report: Vec<i32>) -> i32 {
    if is_safe(&report) {
        return 0;
    }
    let (_, mut levels) = report.split_first_mut().unwrap();
    if is_safe(&levels.to_vec()) {
        return 1;
    }
    (_, levels) = report.split_last_mut().unwrap();
    if is_safe(&levels.to_vec()) {
        return 1;
    }
    for i in 0..report.len() {
        let mut report_without_wrong_level = report.clone();
        report_without_wrong_level.remove(i);
        if is_safe(&report_without_wrong_level) {
            return 1;
        }
        println!("Report is still wrong: {:?}", report_without_wrong_level)
    }
    return 2;
}

fn main() {
    let mut count_safe: i32 = 0;
    let mut count_dampener_safe: i32 = 0;
    let mut unsafe_reports: i32 = 0;
    let mut nb_reports: i32 = 0;
    if let Ok(reports) = read_lines("./input") {
        // Consumes the iterator, returns an (Optional) String
        for report in reports.flatten() {
            let levels: Vec<i32> = report
                .split_whitespace()
                .map(|l| l.parse::<i32>().unwrap())
                .collect();
            match problem_dampener(levels) {
                0 => {
                    count_safe += 1;
                    count_dampener_safe += 1
                }
                1 => count_dampener_safe += 1,
                _ => unsafe_reports += 1,
            }
            nb_reports += 1;
        }
        println!("Total reports: {}", nb_reports);
        println!("Safe reports: {}", count_safe);
        println!(
            "Safe reports after Problem Dampener: {}",
            count_dampener_safe
        );
        println!("Unsafe reports: {}", unsafe_reports);
    }
}
