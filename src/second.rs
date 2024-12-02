#[path = "utils.rs"]
mod utils;

pub fn check_safety(input: &str) -> i32 {
    let cleaned: Vec<Vec<i32>> = utils::string_to_2d_array(input, manipulate_input);
    cleaned.iter().fold(0, |acc, x| acc + x[0])
}

pub fn check_safety_dampened(input: &str) -> i32 {
    let cleaned: Vec<Vec<i32>> = utils::string_to_2d_array(input, safety_dampener);
    cleaned.iter().fold(0, |acc, x| acc + x[0])
}

fn manipulate_input(original: &str) -> Vec<i32> {
    let parsed: Vec<i32> = original
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let diffs = report_diffs(&parsed);
    is_safe(&diffs)
}

fn safety_dampener(original: &str) -> Vec<i32> {
    let parsed: Vec<i32> = original
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    can_be_safe(&parsed)
}

fn report_diffs(report: &Vec<i32>) -> Vec<i32> {
    let mut diffs = Vec::with_capacity(report.len() - 1);

    for window in report.windows(2) {
        diffs.push(window[1] - window[0]);
    }
    diffs
}

fn can_be_safe(report: &Vec<i32>) -> Vec<i32> {
    if is_safe(&report_diffs(report)) == vec![1] {
        return vec![1];
    }

    let sub_vectors: Vec<Vec<i32>> = (0..report.len())
        .map(|i| {
            let mut sub_vec = report.clone();
            sub_vec.remove(i);
            report_diffs(&sub_vec)
        })
        .collect();
    for vector in sub_vectors {
        if is_safe(&vector) == vec![1] {
            return vec![1];
        }
    }
    vec![0]
}

fn is_safe(report: &Vec<i32>) -> Vec<i32> {
    let safe = report.iter().all(|&x| x.signum() == report[0].signum())
        && report
            .iter()
            .map(|&x| x.abs())
            .min()
            .map_or(false, |min| min >= 1)
        && report
            .iter()
            .map(|&x| x.abs())
            .max()
            .map_or(false, |max| max <= 3);
    if safe {
        return vec![1];
    } else {
        vec![0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_safety() {
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

        let result = check_safety(input);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_check_safety_dampened() {
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

        let result = check_safety_dampened(input);
        assert_eq!(result, 4);
    }
}
