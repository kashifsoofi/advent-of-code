use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let fully_contained_pair_count = part1_get_fully_contained_pair_count();
    let overlapping_count = part2_overlapping_pairs_count();
    println!("fully contained pairs: {}, overlappint pairs: {}", fully_contained_pair_count, overlapping_count);
}

fn part1_get_fully_contained_pair_count() -> i32 {
    let file = File::open("../../inputs/day04.txt").expect("unable to open input file");
    let reader = BufReader::new(file);

    let mut pairs = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        if is_pair_fully_contained(line) {
            pairs += 1
        }
    }

    pairs
}

fn is_pair_fully_contained(line: String) -> bool {
    let pairs: Vec<&str> = line.split(",").collect();

    let limits1: Vec<i32> = pairs[0].split("-").map(|x| x.parse::<i32>().unwrap()).collect();
    let limits2: Vec<i32> = pairs[1].split("-").map(|x| x.parse::<i32>().unwrap()).collect();

    if limits1[0] >= limits2[0] && limits1[1] <= limits2[1] {
        return true;
    }

    if limits2[0] >= limits1[0] && limits2[1] <= limits1[1] {
        return true;
    }

    return false;
}

fn part2_overlapping_pairs_count() -> i32 {
    let file = File::open("../../inputs/day04.txt").expect("unable to open input file");
    let reader = BufReader::new(file);

    let mut count = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        if is_pair_overlaps(line) {
            count += 1
        }
    }

    count
}

fn is_pair_overlaps(line: String) -> bool {
    let pairs: Vec<&str> = line.split(",").collect();

    let limits1: Vec<i32> = pairs[0].split("-").map(|x| x.parse::<i32>().unwrap()).collect();
    let limits2: Vec<i32> = pairs[1].split("-").map(|x| x.parse::<i32>().unwrap()).collect();

    if (limits2[0] >= limits1[0] && limits2[0] <= limits1[1]) ||
        (limits2[1] >= limits1[0] && limits2[1] <= limits1[1]) {
        return true
    }

    if (limits1[0] >= limits2[0] && limits1[0] <= limits2[1]) ||
        (limits1[1] >= limits2[0] && limits1[1] <= limits2[1]) {
        return true
    }

    return false;
}

#[cfg(test)]
mod tests {
    use crate::is_pair_fully_contained;
    use crate::is_pair_overlaps;

    #[test]
    fn first_pair_fully_contained_should_return_true() {
        let contained = is_pair_fully_contained("6-6,4-6".to_string());
        assert_eq!(contained, true);
    }

    #[test]
    fn second_pair_fully_contained_should_return_true() {
        let contained = is_pair_fully_contained("2-8,3-7".to_string());
        assert_eq!(contained, true);
    }

    #[test]
    fn single_section_overlaps_should_return_true() {
        let contained = is_pair_overlaps("5-7,7-9".to_string());
        assert_eq!(contained, true);

        let contained = is_pair_overlaps("4-6,6-6".to_string());
        assert_eq!(contained, true);

        let contained = is_pair_overlaps("6-6,4-6".to_string());
        assert_eq!(contained, true);
    }

    #[test]
    fn all_sections_overlap_should_return_true() {
        let contained = is_pair_overlaps("2-8,3-7".to_string());
        assert_eq!(contained, true);

        let contained = is_pair_overlaps("3-7,2-8".to_string());
        assert_eq!(contained, true);
    }

    #[test]
    fn partial_section_overlaps_should_return_true() {
        let contained = is_pair_overlaps("2-6,4-8".to_string());
        assert_eq!(contained, true);

        let contained = is_pair_overlaps("4-8,2-6".to_string());
        assert_eq!(contained, true);
    }
}