#[path = "utils.rs"]
mod utils;
use std::collections::HashSet;

pub fn add_mult(input: &str) -> u128 {
    let parsed = utils::string_to_2d_array(input, result_then_inputs);
    let structured: Vec<(u128, Vec<u128>)> = parsed.iter().map(|v| prepare(v)).collect();
    let results: Vec<u128> = structured
        .iter()
        .map(|(r, i)| do_the_math(r, i, false))
        .collect();
    results.iter().sum()
}

pub fn concat_add_mult(input: &str) -> u128 {
    let parsed = utils::string_to_2d_array(input, result_then_inputs);
    let structured: Vec<(u128, Vec<u128>)> = parsed.iter().map(|v| prepare(v)).collect();
    let results: Vec<u128> = structured
        .iter()
        .map(|(r, i)| do_the_math(r, i, true))
        .collect();
    println!("{:?}", results);
    results.iter().sum()
}

fn do_the_math(target: &u128, numbers: &[u128], allow_concats: bool) -> u128 {
    // Dynamic programming, another new concept learned!
    let mut states = HashSet::new();
    states.insert(numbers[0]);

    for &num in &numbers[1..] {
        let mut next_states = HashSet::new();
        for &value in &states {
            next_states.insert(value + num);
            next_states.insert(value * num);
            if allow_concats {
                next_states.insert(concatenate(value, num));
            }
        }
        states = next_states;
    }

    if states.contains(&target) {
        *target
    } else {
        0
    }
}

fn concatenate(a: u128, b: u128) -> u128 {
    let concatenated = format!("{}{}", a, b);
    concatenated.parse::<u128>().unwrap()
}

fn prepare(input: &Vec<u128>) -> (u128, Vec<u128>) {
    (*input.first().unwrap(), input[1..].to_vec())
}

fn result_then_inputs(string: &str) -> Vec<u128> {
    let mut collected: Vec<u128> = Vec::new();
    let mut line = string.split(":");
    let expected = line
        .next()
        .and_then(|s| s.parse::<u128>().ok())
        .unwrap_or(0);
    collected.push(expected);
    let inputs = line
        .next()
        .map(|s| {
            s.split_whitespace()
                .filter_map(|num| num.parse::<u128>().ok())
                .collect::<Vec<u128>>()
        })
        .unwrap_or_else(Vec::new);
    collected.extend(inputs);
    collected
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_mult() {
        let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

        let result = add_mult(input);
        assert_eq!(result, 3749);
    }

    #[test]
    fn test_concat_add_mult() {
        let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

        let result = concat_add_mult(input);
        assert_eq!(result, 11387);
    }
}
