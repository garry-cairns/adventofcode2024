#[path = "utils.rs"]
mod utils;
use core::ops::Range;
use regex::Captures;
use regex::Regex;

pub fn uncorrupt(input: &str) -> i32 {
    let rx = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let captures: Vec<Captures> = rx.captures_iter(input).collect();
    captures.iter().fold(0, |acc, x| acc + do_mult(x))
}

pub fn dodont(input: &str) -> i32 {
    let dont_indices: Vec<usize> = input
        .match_indices("don't()")
        .map(|(index, _)| index)
        .collect();
    let do_indices: Vec<usize> = input
        .match_indices("do()")
        .map(|(index, _)| index)
        .collect();
    let ranges = construct_ranges(dont_indices, do_indices, input.len());
    let mut cleaned = String::new();
    for range in ranges {
        cleaned.push_str(&input[range.clone()]);
    }
    let rx = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let captures: Vec<Captures> = rx.captures_iter(&cleaned).collect();
    captures.iter().fold(0, |acc, x| acc + do_mult(x))
}

fn construct_ranges(donts: Vec<usize>, dos: Vec<usize>, max: usize) -> Vec<Range<usize>> {
    let mut ranges = Vec::new();
    let final_do = dos.last().unwrap();
    let final_dont = donts.last().unwrap();
    let stopping_point = if final_dont < final_do {
        max
    } else {
        *final_dont
    };
    if !donts.is_empty() {
        ranges.push(0..donts[0]);
    }
    let mut last_pushed_dont = donts[0];
    if !dos.is_empty() {
        for do_index in dos {
            if do_index >= stopping_point {
            } else if do_index < last_pushed_dont {
            } else {
                let dont = donts.iter().find(|&x| x > &do_index).unwrap_or(&max);
                ranges.push(do_index..*dont);
                last_pushed_dont = *dont;
            }
        }
    }
    ranges
}

fn do_mult(captures: &Captures) -> i32 {
    let left: i32 = captures[1].parse().unwrap();
    let right: i32 = captures[2].parse().unwrap();
    left * right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uncorrupt() {
        let input = r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;

        let result = uncorrupt(input);
        assert_eq!(result, 161);
    }

    #[test]
    fn test_dodont() {
        let input = r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;

        let result = dodont(input);
        assert_eq!(result, 48);
    }
}
