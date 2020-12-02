use super::Policy;

pub struct NewPolicy {
    symbol: u8,
    first_pos: usize,
    second_pos: usize,
}

impl Policy for NewPolicy {
    fn from_str(input: &str) -> Self {
        NewPolicy::from(input)
    }

    fn is_password_valid(&self, pw: &str) -> bool {
        let bytes = pw.as_bytes();
        (bytes[self.first_pos] == self.symbol) ^ (bytes[self.second_pos] == self.symbol)
    }
}

impl From<&str> for NewPolicy {
    fn from(s: &str) -> Self {
        let parts = s.split_whitespace().collect::<Vec<_>>();

        let pos_parts: Vec<usize> = parts[0].split('-').map(|n| n.parse().unwrap()).collect();

        NewPolicy {
            symbol: parts[1].as_bytes()[0],
            first_pos: pos_parts[0] - 1,
            second_pos: pos_parts[1] - 1,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn valid() {
        let policy = NewPolicy::from_str("1-3 a");
        assert!(policy.is_password_valid("abcde"));
    }

    #[test]
    fn invalid() {
        let policy = NewPolicy::from_str("1-3 b");
        assert!(!policy.is_password_valid("cdefg"));

        let policy = NewPolicy::from_str("2-9 c");
        assert!(!policy.is_password_valid("ccccccccc"));
    }
}
