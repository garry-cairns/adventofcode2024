#[path = "utils.rs"]
mod utils;
use std::collections::HashMap;

pub fn check_distance(input: &str) -> u32 {
    let cleaned: (Vec<u32>, Vec<u32>) = string_to_2d_array(input);
    let differences: Vec<u32> = cleaned
        .0
        .iter()
        .zip(cleaned.1.iter())
        .map(|(&x, &y)| difference(x, y))
        .collect();
    differences.iter().sum()
}

pub fn check_similarity(input: &str) -> u32 {
    let cleaned: (Vec<u32>, Vec<u32>) = string_to_2d_array(input);
    let frequency = count_occurrences(cleaned.1);
    cleaned
        .0
        .iter()
        .fold(0, |acc, &x| acc + x * frequency.get(&x).unwrap_or(&0))
}

fn manipulate_input(original: &str) -> (u32, u32) {
    let mut updated = original.split_whitespace();

    let first_num = updated
        .next()
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(0);
    let second_num = updated
        .next()
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(0);

    (first_num, second_num)
}

fn string_to_2d_array(input: &str) -> (Vec<u32>, Vec<u32>) {
    let initial: Vec<(u32, u32)> = input.lines().map(|line| manipulate_input(line)).collect();
    let mut first_column: Vec<u32> = initial.iter().map(|(x, _)| *x).collect();
    let mut second_column: Vec<u32> = initial.iter().map(|(_, y)| *y).collect();
    first_column.sort();
    second_column.sort();
    (first_column, second_column)
}

fn difference(first: u32, second: u32) -> u32 {
    first.abs_diff(second)
}

fn count_occurrences<T: Eq + std::hash::Hash>(items: Vec<T>) -> HashMap<T, u32> {
    let mut counter = HashMap::new();

    for item in items {
        *counter.entry(item).or_insert(0) += 1;
    }

    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_distance() {
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
        let result = check_distance(input);
        assert_eq!(result, 11);
    }

    #[test]
    fn test_check_similarity() {
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
        let result = check_similarity(input);
        assert_eq!(result, 31);
    }
}
