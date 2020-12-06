const INPUT: &'static str = include_str!("../input.txt");
fn main() {
    let mut input_lines = INPUT.lines().map(|b| find_seat_id(b)).collect::<Vec<_>>();
    input_lines.sort_unstable();

    let p1 = part_1(&input_lines);
    println!("Part 1: {}", p1);

    let p2 = part_2(&input_lines);
    println!("Part 1: {}", p2);
}

fn part_1(seat_ids: &[usize]) -> usize {
    *seat_ids.iter().last().unwrap()
}

fn part_2(seat_ids: &[usize]) -> usize {
    seat_ids.windows(2).find(|&w| w[1] != w[0] + 1).unwrap()[0] + 1
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
