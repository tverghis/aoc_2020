mod rule;

use rule::{InvertedRules, ParsedRule, Rules};

const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    let rules = parse_rules(INPUT);

    let p1 = part_1(&rules);
    println!("Part 1: {}", p1);

    let p2 = part_2(&rules);
    println!("Part 2: {}", p2);
}

fn part_1(parsed_rules: &[ParsedRule]) -> usize {
    let mut rules = Rules::new();

    for p in parsed_rules {
        rules.insert_parsed(p);
    }

    rules.containers_for("shiny gold").len()
}

fn part_2(parsed_rules: &[ParsedRule]) -> u32 {
    let mut rules = InvertedRules::new();

    for p in parsed_rules {
        rules.insert_parsed(p);
    }

    rules.required_inside("shiny gold")
}

fn parse_rules(input: &str) -> Vec<ParsedRule> {
    input.lines().map(|l| l.into()).collect()
}
