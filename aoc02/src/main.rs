use std::{convert::From, ops::RangeInclusive};

const INPUT: &'static str = include_str!("../input.txt");

struct Policy {
    symbol: char,
    range: RangeInclusive<usize>,
}

impl<'a> Policy {
    fn is_password_valid(&self, pw: &'a str) -> bool {
        self.range.contains(&pw.matches(self.symbol).count())
    }
}

impl From<&str> for Policy {
    fn from(s: &str) -> Self {
        let parts = s.split_whitespace().collect::<Vec<_>>();

        let range_parts = parts[0]
            .split('-')
            .map(|n| n.parse().unwrap())
            .collect::<Vec<_>>();

        Policy {
            symbol: parts[1].as_bytes()[0] as char,
            range: RangeInclusive::new(range_parts[0], range_parts[1]),
        }
    }
}

fn main() {
    let p1 = part_1();
    println!("Part 1: {}", p1);
}

fn part_1() -> usize {
    INPUT
        .lines()
        .map(|l| l.split(':').collect::<Vec<_>>())
        .map(|parts| (Policy::from(parts[0]), parts[1].trim()))
        .filter(|(policy, pw)| policy.is_password_valid(pw))
        .count()
}
