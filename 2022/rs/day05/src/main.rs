use regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() {
    let top_crates = part1_get_top_crates();
    println!(
        "top crates: {}, overlappint pairs: {}",
        top_crates, top_crates
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

fn pad_string(s: &str) -> String {
    if s.starts_with("    ") {
        let s = s.replacen("    ", "xxx ", 1);
        let s = s.replace("    ", " xxx");
        return s.to_string();
    }

    let s = s.replace("    ", " xxx");
    return s.to_string();
}

struct Input {
    stacks: Vec<Vec<String>>,
    instructions: Vec<Instruction>,
}

fn parse_input() -> Input {
    let file = File::open("../../inputs/day05.txt").expect("unable to open input file");
    let reader = BufReader::new(file);

    let pad = "xxx";
    let mut stacks: Vec<Vec<String>> = Vec::new();
    let mut instructions: Vec<Instruction> = Vec::new();

    let mut parse_instructions = false;
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() || line.starts_with(" 1") {
            parse_instructions = true;
            continue;
        }

        if !parse_instructions {
            let line = pad_string(line.as_str());
            let crates: Vec<&str> = line.split_whitespace().collect();
            if stacks.len() == 0 {
                for _ in 0..crates.len() {
                    stacks.push(Vec::new());
                }
            }

            let mut i = 0;
            for c in crates {
                if c.ne(pad) {
                    stacks[i].push(c.to_string());
                }
                i += 1;
            }
        } else {
            let instruction = line.parse::<Instruction>().unwrap();
            instructions.push(instruction);
        }
    }

    for s in stacks.iter_mut() {
        s.reverse();
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
            let c = input.stacks[instruction.from - 1].pop();
            input.stacks[instruction.to - 1].push(c.unwrap().to_string());
        }
    }

    let mut top_crates: String = "".to_owned();
    for i in 0..input.stacks.len() {
        let s = input.stacks[i][input.stacks[i].len() -1].to_owned();
        top_crates.push_str(&s);
    }
    return top_crates.to_string();
}
