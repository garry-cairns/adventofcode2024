#[path = "utils.rs"]
mod utils;

pub fn checksum(input: &str) -> usize {
    let (mut big_string, check_size) = clean(input);
    let mut backfills: Vec<String> = big_string
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

        let result = checksum(input);
        assert_eq!(result, 2858);
    }
}
