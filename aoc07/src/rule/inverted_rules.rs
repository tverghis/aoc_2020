use super::{parsed_rule::ContainedBag, ParsedRule};
use std::collections::HashMap;

#[derive(Debug)]
pub struct InvertedRules {
    contained_bags: HashMap<String, Vec<ContainedBag>>,
}

impl InvertedRules {
    pub fn new() -> Self {
        Self {
            contained_bags: HashMap::new(),
        }
    }

    pub fn insert_parsed(&mut self, parsed_rule: &ParsedRule) {
        for (child, card) in parsed_rule.children.iter() {
            self.insert(parsed_rule.parent.clone(), (child.clone(), *card));
        }
    }

    pub fn insert(&mut self, parent_bag: String, (child, cardinality): (String, u32)) {
        self.contained_bags
            .entry(parent_bag)
            .or_default()
            .push((child, cardinality));
    }

    pub fn required_inside(&self, parent_bag: &str) -> u32 {
        match self.contained_bags.get(parent_bag) {
            Some(children) => children
                .iter()
                .fold(0, |acc, (c, n)| acc + (n + (n * self.required_inside(c)))),
            None => 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_required_inside() {
        let mut rules = InvertedRules::new();
        rules.insert("shiny gold".into(), ("dark red".into(), 2));
        rules.insert("dark red".into(), ("dark orange".into(), 2));
        rules.insert("dark orange".into(), ("dark yellow".into(), 2));
        rules.insert("dark yellow".into(), ("dark green".into(), 2));
        rules.insert("dark green".into(), ("dark blue".into(), 2));
        rules.insert("dark blue".into(), ("dark violet".into(), 2));

        assert_eq!(rules.required_inside("dark blue"), 2);
        assert_eq!(rules.required_inside("shiny gold"), 126);
    }
}
