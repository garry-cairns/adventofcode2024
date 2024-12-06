#[path = "utils.rs"]
mod utils;
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Rule {
    left: u32,
    right: u32,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct RuleSet {
    rules: Vec<Rule>,
}

impl RuleSet {
    fn lefts(&self, key: u32) -> HashSet<&u32> {
        let mut left_values: HashSet<&u32> = HashSet::new();
        for rule in &self.rules {
            if rule.right == key {
                left_values.insert(&rule.left);
            }
        }
        left_values
    }

    fn rights(&self, key: u32) -> HashSet<&u32> {
        let mut right_values: HashSet<&u32> = HashSet::new();
        for rule in &self.rules {
            if rule.left == key {
                right_values.insert(&rule.right);
            }
        }
        right_values
    }
}

pub fn pageorderingtotal(input: &str) -> u32 {
    let sections: Vec<&str> = input.split("\n\n").collect();
    let rules = utils::string_to_2d_array(sections[0], make_rules).concat();
    let pages = utils::string_to_2d_array(sections[1], extract_page_numbers);
    pages.iter().fold(0, |acc, x| acc + is_valid(&x, &rules))
}

pub fn corrected_total(input: &str) -> u32 {
    let sections: Vec<&str> = input.split("\n\n").collect();
    let rules = utils::string_to_2d_array(sections[0], make_rules).concat();
    let pages = utils::string_to_2d_array(sections[1], extract_page_numbers);
    pages.iter().fold(0, |acc, x| acc + valid_perm(&x, &rules))
}

fn valid_perm(page: &Vec<u32>, rules: &Vec<Rule>) -> u32 {
    if is_valid(page, rules) != 0 {
        return 0;
    } else {
        let relevant = relevant_rules(page, &rules);
        for (i, key) in page.iter().enumerate() {
            let lefts = relevant.lefts(*key);
            let left_set = HashSet::from_iter(&page[..i]);
            let left_intersection: HashSet<&u32> = lefts.intersection(&left_set).cloned().collect();
            let rights = relevant.rights(*key);
            let right_set = HashSet::from_iter(&page[i + 1..]);
            let right_intersection: HashSet<&u32> =
                rights.intersection(&right_set).cloned().collect();
            return 1;
        }
    }
    return 0;
}

fn is_valid(page: &Vec<u32>, rules: &Vec<Rule>) -> u32 {
    let relevant = relevant_rules(page, &rules);
    for (i, key) in page.iter().enumerate() {
        let lefts = relevant.lefts(*key);
        let left_set = HashSet::from_iter(&page[..i]);
        let left_intersection: HashSet<&u32> = lefts.intersection(&left_set).cloned().collect();
        let rights = relevant.rights(*key);
        let right_set = HashSet::from_iter(&page[i + 1..]);
        let right_intersection: HashSet<&u32> = rights.intersection(&right_set).cloned().collect();
        if left_set == left_intersection && right_set == right_intersection {
        } else {
            return 0;
        }
    }
    page[page.len() / 2]
}

fn extract_page_numbers(original: &str) -> Vec<u32> {
    original.split(",").map(|s| s.parse().unwrap()).collect()
}

fn make_rules(original: &str) -> Vec<Rule> {
    let parsed: Vec<u32> = original.split("|").map(|s| s.parse().unwrap()).collect();
    [Rule {
        left: parsed[0],
        right: parsed[1],
    }]
    .to_vec()
}

fn relevant_rules(page: &Vec<u32>, rules: &Vec<Rule>) -> RuleSet {
    let mut relevant: Vec<Rule> = Vec::new();
    let mut left: HashMap<u32, Vec<Rule>> = HashMap::new();
    let mut right: HashMap<u32, Vec<Rule>> = HashMap::new();
    for rule in rules {
        left.entry(rule.left.clone())
            .or_insert_with(Vec::new)
            .push(rule.clone());

        right
            .entry(rule.right.clone())
            .or_insert_with(Vec::new)
            .push(rule.clone());
    }
    for item in page {
        relevant.extend(left.get(&item).unwrap_or(&vec![]).clone());
        relevant.extend(right.get(&item).unwrap_or(&vec![]).clone());
    }
    RuleSet { rules: relevant }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wordsearch() {
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

        let result = pageorderingtotal(input);
        assert_eq!(result, 143);
    }

    #[test]
    fn test_corrected() {
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

        let result = corrected_total(input);
        assert_eq!(result, 123);
    }
}
