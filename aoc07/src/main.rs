mod rule;

use rule::{ParsedRule, Rules};

const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    let rules = parse_rules(INPUT);

    let p1 = part_1(&rules);
    println!("Part 1: {}", p1);
}

fn part_1(rules: &Rules) -> usize {
    rules.containers_for("shiny gold").len()
}

fn parse_rules(input: &str) -> Rules {
    let mut rules = Rules::new();

    let parsed: Vec<ParsedRule> = input.lines().map(|l| l.into()).collect();
    for p in parsed {
        rules.insert_parsed(p);
    }

    rules
}
