use std::collections::VecDeque;

fn main() {
    let first_start_of_packet_marker = part1_get_first_start_of_packet_marker();
    // let overlapping_count = part2_overlapping_pairs_count();
    println!("first start-of-packet marker: {}, overlappint pairs: {}", first_start_of_packet_marker, first_start_of_packet_marker);
}

fn get_input() -> Vec<char> {
    let content = std::fs::read_to_string("../../inputs/day06.txt").expect("unable to open input file");
    content.chars().collect()
}

fn part1_get_first_start_of_packet_marker() -> u32 {
    let chars = get_input();

    let mut vals = VecDeque::new();
    let mut index = 0;
    for char in chars {
        vals.push_back(char);
        index += 1;
        
        if index < 4 {
            continue;
        }
        
        let mut has_duplicate = false;
        for i in 0..3 {
            for j in (i+1)..4 {
                if vals[i] == vals[j] {
                    has_duplicate = true;
                    break;
                }
            }
            if has_duplicate {
                break;
            }
        }
        
        if !has_duplicate {
            break;
        }
        
        if has_duplicate {
            vals.pop_front();
        }
    }
    index
}
