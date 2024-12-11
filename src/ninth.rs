#[path = "utils.rs"]
mod utils;
use std::collections::HashSet;

pub fn checksum(input: &str) -> usize {
    let (big_string, check_size) = clean(input);
    let backfills: Vec<String> = big_string
        .clone()
        .iter()
        .cloned()
        .filter(|s| *s != ".")
        .rev()
        .collect();
    let mut backfill_index = 0;
    let mut checksum: Vec<usize> = Vec::new();
    for (i, _) in (0..check_size).enumerate() {
        let existing = big_string.get(i).unwrap().parse();
        match existing {
            Ok(num) => checksum.push(num),
            Err(_) => {
                let backfill: usize = backfills.get(backfill_index).unwrap().parse().unwrap();
                checksum.push(backfill);
                backfill_index += 1;
            }
        }
    }
    checksum
        .iter()
        .enumerate()
        .fold(0, |acc, (index, &value)| acc + index * value)
}

pub fn whole_files(input: &str) -> usize {
    let (mut big_string, check_size) = clean(input);
    let mut backfills: Vec<String> = big_string
        .clone()
        .iter()
        .cloned()
        .filter(|s| *s != ".")
        .collect();
    let distinct: HashSet<String> = backfills.iter().cloned().collect();
    let mut distinct_ordered: Vec<String> = distinct.into_iter().collect();
    distinct_ordered.sort_by(|a, b| b.cmp(a));
    for fig in distinct_ordered {
        let first_value_index = big_string.iter().position(|s| *s == fig);
        match first_value_index {
            Some(value_index) => {
                let backfill: Vec<String> = backfills
                    .iter()
                    .filter(|&s| *s == fig)
                    .map(|s| s.clone())
                    .collect();
                if backfill.len() > 0 {
                    let first_fill_index = big_string
                        .windows(backfill.len())
                        .position(|window| window.iter().all(|s| s == "."));
                    match first_fill_index {
                        Some(fill_index) => {
                            if fill_index < value_index {
                                let matching_indices: Vec<usize> = big_string
                                    .windows(backfill.len())
                                    .enumerate()
                                    .filter(|(i, window)| *window == backfill.as_slice())
                                    .map(|(i, _)| i)
                                    .collect();
                                if matching_indices.len() == 1 {
                                    big_string.splice(
                                        fill_index..(fill_index + backfill.len()),
                                        backfill.clone(),
                                    );
                                    let replacement = vec![String::from("."); backfill.len()];
                                    big_string.splice(
                                        matching_indices[0]..(matching_indices[0] + backfill.len()),
                                        replacement,
                                    );
                                }
                            }
                        }
                        None => {}
                    }
                }
            }
            None => {}
        }
    }
    big_string
        .iter()
        .enumerate()
        .fold(0, |acc, (index, &ref value)| {
            acc + safe_product(index, value.to_string())
        })
}

fn safe_product(index: usize, value: String) -> usize {
    match value.parse::<usize>() {
        Ok(i) => index * i,
        Err(_) => 0,
    }
}

fn clean(input: &str) -> (Vec<String>, usize) {
    let mut big_string: Vec<String> = Vec::new();
    let mut check_size = 0;
    let mut fid = 0;

    for (i, c) in input.trim().chars().enumerate() {
        if i % 2 == 0 {
            let file = c.to_digit(10);
            match file {
                Some(num) => {
                    for _ in 0..num {
                        big_string.push(fid.to_string());
                        check_size += 1;
                    }
                    fid += 1;
                }
                None => {}
            }
        } else {
            let dig = c.to_digit(10);
            match dig {
                Some(num) => {
                    for _ in 0..num {
                        big_string.push(".".to_string());
                    }
                }
                None => println!("Character {:?} at index {:?}", c, i),
            }
        }
    }
    (big_string, check_size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checksum() {
        let input = r#"2333133121414131402"#;

        let result = checksum(input);
        assert_eq!(result, 1928);
    }

    #[test]
    fn test_whole_files() {
        let input = r#"2333133121414131402"#;

        let result = whole_files(input);
        assert_eq!(result, 2858);
    }

    #[test]
    fn test_whole_files_again() {
        let input = r#"2333133121414131401"#;

        let result = whole_files(input);
        assert_eq!(result, 2746);
    }
}

/*
00...111...2...333.44.5555.6666.777.888899
0099.111...2...333.44.5555.6666.777.8888..
0099.1117772...333.44.5555.6666.....8888..
0099.111777244.333....5555.6666.....8888..
00992111777.44.333....5555.6666.....8888..
*/
