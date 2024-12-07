use log::debug;
use reqwest::blocking::get;
use reqwest::Error;
use std::fs::File;
use std::io::{self, Read};

pub fn url_to_string(url: &str) -> Result<String, Error> {
    debug!("URL is {url}");
    let response = get(url)?;
    let status = response.status();
    debug!("Status is {status}");

    if response.status().is_success() {
        let body = response.text()?;
        debug!("Body is {body}");
        Ok(body)
    } else {
        Ok(String::new())
    }
}

pub fn file_input(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn string_to_2d_array<F, T>(input: &str, manipulation: F) -> Vec<Vec<T>>
where
    F: Fn(&str) -> Vec<T>,
{
    input.lines().map(|line| manipulation(line)).collect()
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct CoOrd {
    pub i: usize,
    pub j: usize,
}

// Assumes you're looking for exactly one `target`
pub fn locate_in_grid<T>(grid: &Vec<Vec<T>>, target: &T) -> Option<CoOrd>
where
    T: Eq,
{
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == *target {
                return Some(CoOrd { i: i, j: j });
            }
        }
    }
    None
}

pub fn just_chars(string: &str) -> Vec<char> {
    string.chars().collect()
}

fn digits(original: &str) -> Vec<i32> {
    original
        .chars()
        .filter(|&c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_url_to_string() {
        let _m = mock("GET", "/")
            .with_status(200)
            .with_body("Hello, world!")
            .create();

        let result = url_to_string(&mockito::server_url()).unwrap();

        assert_eq!(result, "Hello, world!");
    }

    #[test]
    fn string_conversion() {
        let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
        let result = string_to_2d_array(input, digits);
        let mut nd_array: Vec<Vec<i32>> = Vec::new();
        nd_array.push(vec![1, 2]);
        nd_array.push(vec![3, 8]);
        nd_array.push(vec![1, 2, 3, 4, 5]);
        nd_array.push(vec![7]);
        assert_eq!(result, nd_array);
    }

    #[test]
    fn test_file_input() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let content = "Hello, world!";
        writeln!(temp_file, "{}", content).unwrap();

        let temp_path = temp_file.path().to_str().unwrap();
        let result = file_input(temp_path).unwrap();

        assert_eq!(result.trim(), content);
    }

    #[test]
    fn test_find_in_grid() {
        let input = r#".....
.....
...g.
....."#;
        let grid = string_to_2d_array(input, just_chars);
        let result = locate_in_grid(&grid, &'g');

        assert_eq!(result.unwrap(), CoOrd { i: 2, j: 3 });
    }
}
