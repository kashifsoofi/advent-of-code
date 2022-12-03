use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let current_floor = part1_get_current_floor();
    let position = part2_get_position();
    println!("current floor: {}, position: {}", current_floor, position);
}

fn part1_get_current_floor() -> i32 {
    let file = File::open("../../inputs/day01.txt").expect("unable to open input file");
    let reader = BufReader::new(file);

    let mut current_floor = 0;
    for line in reader.lines() {
        for ch in line.expect("unable to read line").chars() {
            if ch == '(' {
                current_floor += 1;
            } else if ch == ')' {
                current_floor -= 1;
            }
        }
    }

    current_floor
}

fn part2_get_position() -> i32 {
    let file = File::open("../../inputs/day01.txt").expect("unable to open input file");
    let reader = BufReader::new(file);

    let mut current_floor = 0;
    let mut position = 1;
    for line in reader.lines() {
        for ch in line.expect("unable to read line").chars() {
            if ch == '(' {
                current_floor += 1;
            } else if ch == ')' {
                current_floor -= 1;
            }

            if current_floor == -1 {
                break;
            }
            position += 1;
        }
    }

    position
}