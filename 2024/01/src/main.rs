use itertools::{Either, Itertools};
use std::fs;

struct Numbers {
    left: Vec<i32>,
    right: Vec<i32>,
}

fn read_file() -> String {
    let file_path: &str = "input";
    println!("Reading file {file_path}");
    return fs::read_to_string(file_path).expect("Should have been able to read the file");
}

fn parse_input(input: &String) -> Numbers {
    let (left, right): (Vec<i32>, Vec<i32>) =
        input
            .split_whitespace()
            .enumerate()
            .partition_map(|(i, d)| {
                if i % 2 == 0 {
                    Either::Left(d.parse::<i32>().unwrap())
                } else {
                    Either::Right(d.parse::<i32>().unwrap())
                }
            });
    return Numbers { left, right };
}

fn part_one(nums: &mut Numbers) {
    let mut distance: i32 = 0;
    nums.left.sort();
    nums.right.sort();
    let sorted = nums.left.iter().zip(&nums.right);
    for (l, r) in sorted {
        if l - r > 0 {
            distance += l - r;
        } else {
            distance += r - l;
        }
    }
    println!("Distance: {distance}");
}

fn part_two(nums: Numbers) {
    let mut similarity: i32 = 0;
    for l in &nums.left {
        let mut dup: i32 = 0;
        for r in &nums.right {
            if l == r {
                dup += 1;
            }
        }
        similarity += l * dup;
    }
    println!("Similarity score: {}", similarity);
}

fn main() {
    let data: String = read_file();
    let mut lists: Numbers = parse_input(&data);
    part_one(&mut lists);
    part_two(lists);
}
