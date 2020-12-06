use std::collections::HashSet;
use std::iter::Extend;

const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    let grouped_answers = INPUT.split("\n\n").collect::<Vec<_>>();

    let p1 = part_1(&grouped_answers);
    println!("{}", p1);

    let p2 = part_2(&grouped_answers);
    println!("{}", p2);
}

fn part_1(grouped_answers: &[&str]) -> usize {
    grouped_answers
        .iter()
        .map(|&group| unique_answers_for_group(group))
        .fold(0, |acc, ans_count| acc + ans_count)
}

fn part_2(grouped_answers: &[&str]) -> usize {
    grouped_answers
        .iter()
        .map(|&group| unique_common_answers_for_group(group))
        .fold(0, |acc, ans_count| acc + ans_count)
}

fn unique_answers_for_group(group_answers: &str) -> usize {
    group_answers
        .lines()
        .fold(HashSet::new(), |mut acc, line| {
            acc.extend(line.chars());
            acc
        })
        .len()
}

fn unique_common_answers_for_group(group_answers: &str) -> usize {
    let answer_sets = group_answers
        .lines()
        .map(|l| l.chars().collect::<HashSet<_>>())
        .collect::<Vec<_>>();

    answer_sets[0]
        .iter()
        .filter(|e| answer_sets.iter().all(|s| s.contains(e)))
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_unique_answers_for_group() {
        assert_eq!(unique_answers_for_group("abcx\nabcy\nabcz"), 6);
        assert_eq!(unique_answers_for_group("abc"), 3);
        assert_eq!(unique_answers_for_group("a\nb\nc"), 3);
        assert_eq!(unique_answers_for_group("ab\nac"), 3);
        assert_eq!(unique_answers_for_group("a\na\na\na"), 1);
        assert_eq!(unique_answers_for_group("b"), 1);
    }

    #[test]
    fn test_unique_common_answers_for_group() {
        assert_eq!(unique_common_answers_for_group("abc"), 3);
        assert_eq!(unique_common_answers_for_group("a\nb\nc"), 0);
        assert_eq!(unique_common_answers_for_group("ab\nac"), 1);
        assert_eq!(unique_common_answers_for_group("a\na\na\na"), 1);
        assert_eq!(unique_common_answers_for_group("b"), 1);
    }
}
