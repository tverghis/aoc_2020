use std::{collections::HashSet, cmp};

const INPUT: &'static str = include_str!("../input.txt");
struct Partition<'a> {
    sequence: &'a str,
    lo_ident: u8,
    hi_ident: u8,
}

impl<'a> Partition<'a> {
    fn new(sequence: &'a str, lo_ident: u8, hi_ident: u8) -> Self {
        Partition { sequence, lo_ident, hi_ident }
    }
}

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

fn find_seat_id(sequence: &str) -> usize {
    let row = find_row(&sequence[0..sequence.len() - 3]);
    let col = find_col(&sequence[sequence.len() - 3..]);
    row * 8 + col
}

fn find_row(sequence: &str) -> usize {
    let partition = Partition::new(sequence, b'F', b'B');
    binspace_partition(0, 127, &partition)
}

fn find_col(sequence: &str) -> usize {
    let partition = Partition::new(sequence, b'L', b'R');
    binspace_partition(0, 7, &partition)
}

fn binspace_partition(min: usize, max: usize, partition: &Partition) -> usize {
    let mut min_row = min;
    let mut max_row = max;

    for p in partition.sequence.bytes() {
        let mid_point = (min_row + max_row) / 2;

        if p == partition.lo_ident {
            max_row = mid_point;
        } else if p == partition.hi_ident {
            min_row = mid_point + 1;
        }
    }

    min_row
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_row() {
        assert_eq!(find_row("FBFBBFF"), 44);
        assert_eq!(find_row("BFFFBBF"), 70);
        assert_eq!(find_row("FFFBBBF"), 14);
        assert_eq!(find_row("BBFFBBF"), 102);
    }

    #[test]
    fn test_find_col() {
        assert_eq!(find_col("RLR"), 5);
        assert_eq!(find_col("RRR"), 7);
        assert_eq!(find_col("RLL"), 4);
    }

    #[test]
    fn test_find_seat_id() {
        assert_eq!(find_seat_id("BFFFBBFRRR"), 567);
    }
}
