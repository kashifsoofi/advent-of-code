use std::collections::VecDeque;
use itertools::Itertools;

fn main() {
    let start_of_packet_marker = part1_get_start_of_packet_marker();
    let start_of_message_marker = part2_get_start_of_message_marker();
    println!("first start-of-packet marker: {}, first start-of-message marker: {}", start_of_packet_marker, start_of_message_marker);
}

fn get_input() -> Vec<char> {
    let content = std::fs::read_to_string("../../inputs/day06.txt").expect("unable to open input file");
    content.chars().collect()
}

fn find_marker(len: usize) -> usize {
    let chars = get_input();

    let mut seen = VecDeque::new();
    for i in 0..len {
        seen.push_back(chars[i as usize]);
    }

    for i in (len+1)..chars.len() {
        if seen.clone().into_iter().unique().count() == len {
            return i;
        }

        seen.pop_front();
        seen.push_back(chars[i as usize]);
    }

    return 0;
}

fn part1_get_start_of_packet_marker() -> usize {
    find_marker(4)
}

fn part2_get_start_of_message_marker() -> usize {
    find_marker(14)
}