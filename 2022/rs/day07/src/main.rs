use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let part1_answer = part1_xxx();
    println!("part 1 answer: {}", part1_answer);
    let part2_answer = part2_xxx();
    println!("part 2 answer: {}", part2_answer);
}

struct Input {

}

impl Input {
    fn load() -> Self {
        let file = File::open("../../inputs/day07.txt").expect("unable to open input file");
        let reader = BufReader::new(file);
    
        for line in reader.lines() {
            let _line = line.unwrap();
        }
    
        return Self {
    
        }    
    }
}

fn part1_xxx() -> u32 {
    let _input = Input::load();
    0
}

fn part2_xxx() -> u32 {
    let _input = Input::load();
    0
}