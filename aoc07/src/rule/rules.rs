use super::ParsedRule;
use std::collections::{HashMap, HashSet};

type ContainedBy = (String, u32);

pub struct Rules {
    contained_bags: HashMap<String, Vec<ContainedBy>>,
}

impl Rules {
    pub fn new() -> Self {
        Self {
            contained_bags: HashMap::new(),
        }
    }

    pub fn insert_parsed(&mut self, parsed_rule: &ParsedRule) {
        for (child, card) in parsed_rule.children.iter() {
            self.insert(child.clone(), (parsed_rule.parent.clone(), *card));
        }
    }

    pub fn insert(&mut self, inner_bag: String, (parent, cardinality): (String, u32)) {
        self.contained_bags
            .entry(inner_bag)
            .or_default()
            .push((parent, cardinality));
    }

    pub fn containers_for(&self, inner_bag: &str) -> HashSet<String> {
        let mut containers = HashSet::new();

        if let Some(v) = self.contained_bags.get(inner_bag) {
            for (parent, _) in v.iter() {
                containers.extend(self.containers_for(parent.as_str()));
                containers.insert(parent.into());
            }
        }

        containers
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_num_containers() {
        let mut rules = Rules::new();
        rules.insert("bright white".into(), ("light red".into(), 1));
        rules.insert("muted yellow".into(), ("light red".into(), 2));
        rules.insert("bright white".into(), ("dark orange".into(), 3));
        rules.insert("muted yellow".into(), ("dark orange".into(), 4));
        rules.insert("shiny gold".into(), ("bright white".into(), 1));
        rules.insert("shiny gold".into(), ("muted yellow".into(), 2));

        assert_eq!(rules.containers_for("light red").len(), 0);
        assert_eq!(rules.containers_for("bright white").len(), 2);
        assert_eq!(rules.containers_for("shiny gold").len(), 4);
    }
}
