mod policy;

use policy::{NewPolicy, OldPolicy, Policy};

const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    let p1 = part_1();
    println!("Part 1: {}", p1);

    let p2 = part_2();
    println!("Part 2: {}", p2);
}

fn part_1() -> usize {
    count_valid_passwords(OldPolicy::from_str)
}

fn part_2() -> usize {
    count_valid_passwords(NewPolicy::from_str)
}

fn count_valid_passwords<P: Policy>(policy_fn: fn(&str) -> P) -> usize {
    INPUT
        .lines()
        .map(|l| l.split(':').collect::<Vec<_>>())
        .map(|parts| (policy_fn(parts[0]), parts[1].trim()))
        .filter(|(policy, pw)| policy.is_password_valid(pw))
        .count()
}
