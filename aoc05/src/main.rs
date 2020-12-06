use std::{collections::HashSet, cmp};

const INPUT: &'static str = include_str!("../input.txt");
fn main() {
    let input_lines = INPUT.lines().collect::<Vec<_>>();

    let p1 = part_1(&input_lines);
    println!("Part 1: {}", p1);

    let p2 = part_2(&input_lines);
    println!("Part 1: {}", p2);
}

fn part_1(boarding_passes: &[&str]) -> usize {
    boarding_passes.iter().map(|b| find_seat_id(b)).max().unwrap()
}

fn part_2(boarding_passes: &[&str]) -> usize {
    let mut min_id = usize::MAX;
    let mut max_id = usize::MIN;

    let mut found_seat_ids = HashSet::new();

    for &b in boarding_passes {
        let id = find_seat_id(b);
        min_id = cmp::min(min_id, id);
        max_id = cmp::max(max_id, id);

        found_seat_ids.insert(id);
    }

    let valid_seat_ids = (min_id..=max_id).collect::<HashSet<_>>();

    // There should be exactly one missing ID!
    *valid_seat_ids.difference(&found_seat_ids).nth(0).unwrap()
}

// The "sequences" (both row and column) are just binary numbers.
// `B` and `R` represent 1s, and `L` and `F` represent 0s.
fn find_seat_id(sequence: &str) -> usize {
    sequence.chars().fold(0, |acc, c| (acc << 1) | if c == 'B' || c == 'R' { 1 } else { 0 })
}   

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_seat_id() {
        assert_eq!(find_seat_id("BFFFBBFRRR"), 567);
        assert_eq!(find_seat_id("FFFBBBFRRR"), 119);
        assert_eq!(find_seat_id("BBFFBBFRLL"), 820);
    }
}
