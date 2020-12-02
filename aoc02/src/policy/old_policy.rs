use std::ops::RangeInclusive;

use super::Policy;

pub struct OldPolicy {
    symbol: char,
    range: RangeInclusive<usize>,
}

impl Policy for OldPolicy {
    fn from_str(input: &str) -> Self {
        OldPolicy::from(input)
    }

    fn is_password_valid(&self, pw: &str) -> bool {
        self.range.contains(&pw.matches(self.symbol).count())
    }
}

impl From<&str> for OldPolicy {
    fn from(s: &str) -> Self {
        let parts = s.split_whitespace().collect::<Vec<_>>();

        let range_parts = parts[0]
            .split('-')
            .map(|n| n.parse().unwrap())
            .collect::<Vec<_>>();

        OldPolicy {
            symbol: parts[1].as_bytes()[0] as char,
            range: RangeInclusive::new(range_parts[0], range_parts[1]),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn valid() {
        let policy = OldPolicy::from_str("1-3 a");
        assert!(policy.is_password_valid("abcde"));

        let policy = OldPolicy::from_str("2-9 c");
        assert!(policy.is_password_valid("ccccccccc"));
    }

    #[test]
    fn invalid() {
        let policy = OldPolicy::from_str("1-3 b");
        assert!(!policy.is_password_valid("cdefg"));
    }
}
