use std::io::{BufRead, BufReader};
use std::fs::File;
use std::str::FromStr;

fn main() {
    let contained_pairs_count = part1_get_contained_pairs_count();
    let overlapping_count = part2_overlapping_pairs_count();
    println!("fully contained pairs: {}, overlappint pairs: {}", contained_pairs_count, overlapping_count);
}

struct Range {
    from: u32,
    to: u32,
}

impl FromStr for Range {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s
            .split_once('-')
            .ok_or_else(|| format!("cannot split range: {}", s))?;
        Ok(Self{
            from: a.parse().map_err(|_| format!("cannot parse: {}", a))?,
            to: b.parse().map_err(|_| format!("cannot parse: {}", b))?,
        })
    }
}

struct Pair {
    first: Range,
    second: Range,
}

impl FromStr for Pair {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s
            .split_once(',')
            .ok_or_else(|| format!("cannot split pairs: {}", s))?;
        Ok(Self{
            first: a.parse()?,
            second: b.parse()?,
        })
    }
}
fn part1_get_contained_pairs_count() -> i32 {
    let file = File::open("../../inputs/day04.txt").expect("unable to open input file");
    let reader = BufReader::new(file);

    let mut pairs = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let pair = line.parse::<Pair>().unwrap();
        if is_contained_pair(pair) {
            pairs += 1
        }
    }

    pairs
}

fn is_contained_pair(pair: Pair) -> bool {
    if (pair.first.from >= pair.second.from && pair.first.to <= pair.second.to) ||
        (pair.second.from >= pair.first.from && pair.second.to <= pair.first.to) {
        return true;
    }

    false
}

fn part2_overlapping_pairs_count() -> i32 {
    let file = File::open("../../inputs/day04.txt").expect("unable to open input file");
    let reader = BufReader::new(file);

    let mut count = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let pair = line.parse::<Pair>().unwrap();
        if is_pair_overlaps(pair) {
            count += 1
        }
    }

    count
}

fn is_pair_overlaps(pair: Pair) -> bool {
    if (pair.second.from >= pair.first.from && pair.second.from <= pair.first.to) ||
        (pair.second.to >= pair.first.from && pair.second.to <= pair.first.to) {
        return true
    }

    if (pair.first.from >= pair.second.from && pair.first.from <= pair.second.to) ||
        (pair.first.to >= pair.second.from && pair.first.to <= pair.second.to) {
        return true
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::Pair;
    use crate::is_contained_pair;
    use crate::is_pair_overlaps;

    #[test]
    fn first_pair_fully_contained_should_return_true() {
        let contained = is_contained_pair("6-6,4-6".to_string().parse::<Pair>().unwrap());
        assert_eq!(contained, true);
    }

    #[test]
    fn second_pair_fully_contained_should_return_true() {
        let contained = is_contained_pair("2-8,3-7".to_string().parse::<Pair>().unwrap());
        assert_eq!(contained, true);
    }

    #[test]
    fn single_section_overlaps_should_return_true() {
        let contained = is_pair_overlaps("5-7,7-9".to_string().parse::<Pair>().unwrap());
        assert_eq!(contained, true);

        let contained = is_pair_overlaps("4-6,6-6".to_string().parse::<Pair>().unwrap());
        assert_eq!(contained, true);

        let contained = is_pair_overlaps("6-6,4-6".to_string().parse::<Pair>().unwrap());
        assert_eq!(contained, true);
    }

    #[test]
    fn all_sections_overlap_should_return_true() {
        let contained = is_pair_overlaps("2-8,3-7".to_string().parse::<Pair>().unwrap());
        assert_eq!(contained, true);

        let contained = is_pair_overlaps("3-7,2-8".to_string().parse::<Pair>().unwrap());
        assert_eq!(contained, true);
    }

    #[test]
    fn partial_section_overlaps_should_return_true() {
        let contained = is_pair_overlaps("2-6,4-8".to_string().parse::<Pair>().unwrap());
        assert_eq!(contained, true);

        let contained = is_pair_overlaps("4-8,2-6".to_string().parse::<Pair>().unwrap());
        assert_eq!(contained, true);
    }
}