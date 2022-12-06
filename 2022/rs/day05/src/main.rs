use regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() {
    let top_crates = part1_get_top_crates();
    let top_crates_with_stacks = part2_get_top_crates_with_stacks();
    println!(
        "top crates: {}, overlappint pairs: {}",
        top_crates, top_crates_with_stacks
    );
}

#[derive(Debug)]
struct Instruction {
    quantity: u32,
    from: usize,
    to: usize,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex::Regex::new(r"move | from | to ").unwrap();
        let parts: Vec<&str> = re.split(s).filter(|x| x.len() > 0).collect();
        if parts.len() != 3 {
            return Err(format!("expected 3 numbers, found: {}", parts.len()));
        }

        Ok(Self {
            quantity: parts[0]
                .parse()
                .map_err(|_| format!("cannot parse: {}", parts[0]))?,
            from: parts[1]
                .parse()
                .map_err(|_| format!("cannot parse: {}", parts[1]))?,
            to: parts[2]
                .parse()
                .map_err(|_| format!("cannot parse: {}", parts[2]))?,
        })
    }
}

struct Input {
    stacks: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
}

fn parse_input() -> Input {
    let file = File::open("../../inputs/day05.txt").expect("unable to open input file");
    let reader = BufReader::new(file);

    let mut stack_lines: Vec<String> = Vec::new();
    let mut move_lines: Vec<String> = Vec::new();

    let mut is_stack_line = true;
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() || line.starts_with(" 1") {
            is_stack_line = false;
            continue;
        }

        if is_stack_line {
            stack_lines.push(line);
        } else {
            move_lines.push(line);
        }
    }

    let mut stacks: Vec<Vec<char>> = Vec::new();
    for line in stack_lines {
        // Add one space to the end of the line
        let line = format!("{} ", line);
        let chunks = line.as_bytes().chunks(4);
        if stacks.len() == 0 {
            for _ in 0..chunks.len() {
                stacks.push(Vec::new());
            }
        }
        let mut stack_index = 0;
        // Read 4 chars at a time
        for chunk in chunks {
            let c = chunk[1] as char;
            if c != ' ' {
                stacks[stack_index].push(c);
            }
            stack_index += 1;
        }
    }

    // Reverse stacks
    for s in stacks.iter_mut() {
        s.reverse();
    }

    let mut instructions: Vec<Instruction> = Vec::new();
    for line in move_lines {
        let instruction = line.parse::<Instruction>().unwrap();
        instructions.push(instruction);
    }

    return Input {
        stacks: stacks,
        instructions: instructions,
    };
}

fn part1_get_top_crates() -> String {
    let mut input = parse_input();

    for instruction in input.instructions {
        for _ in 0..instruction.quantity {
            let c = input.stacks[instruction.from - 1].pop().unwrap();
            input.stacks[instruction.to - 1].push(c);
        }
    }

    let mut top_crates = String::new();
    for i in 0..input.stacks.len() {
        let c = input.stacks[i][input.stacks[i].len() - 1];
        top_crates.push(c);
    }
    return top_crates.to_string();
}

fn part2_get_top_crates_with_stacks() -> String {
    let mut input = parse_input();

    for instruction in input.instructions {
        let mut move_statck = Vec::new();
        for _ in 0..instruction.quantity {
            let c = input.stacks[instruction.from - 1].pop().unwrap();
            move_statck.push(c);
        }

        move_statck.reverse();
        for c in move_statck {
            input.stacks[instruction.to - 1].push(c);
        }
    }
    
    let mut top_crates = String::new();
    for i in 0..input.stacks.len() {
        let c = input.stacks[i][input.stacks[i].len() - 1];
        top_crates.push(c);
    }
    return top_crates.to_string();
}
