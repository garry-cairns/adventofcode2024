use log::debug;
use ndarray::Array2;
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

pub fn vec_to_array2<T>(vec_of_vecs: Vec<Vec<T>>) -> Array2<T>
where
    T: Clone,
{
    let flattened: Vec<T> = vec_of_vecs.clone().into_iter().flatten().collect();
    let rows = flattened.len() / vec_of_vecs.len();
    Array2::from_shape_vec((vec_of_vecs.len(), rows), flattened).unwrap()
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

// Assumes you're looking for all matches of `target`
pub fn locate_all_in_grid<T>(grid: &Array2<T>, target: &T) -> Vec<CoOrd>
where
    T: PartialEq,
{
    grid.indexed_iter()
        .filter(|(_, value)| *value == target)
        .map(|((i, j), _)| CoOrd { i, j })
        .collect()
}

// Assumes you're looking for exactly one `target`
pub fn locate_in_grid<T>(grid: &Array2<T>, target: &T) -> Option<CoOrd>
where
    T: PartialEq,
{
    grid.indexed_iter()
        .find(|(_, value)| *value == target)
        .map(|((i, j), _)| CoOrd { i, j })
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
    fn test_find_all_in_grid() {
        let input = r#".....
.g...
...g.
....."#;
        let grid = vec_to_array2(string_to_2d_array(input, just_chars));
        let result = locate_all_in_grid(&grid, &'g');

        assert_eq!(result, vec![CoOrd { i: 1, j: 1 }, CoOrd { i: 2, j: 3 }]);
    }

    #[test]
    fn test_find_in_grid() {
        let input = r#".....
.....
...g.
....."#;
        let grid = vec_to_array2(string_to_2d_array(input, just_chars));
        let result = locate_in_grid(&grid, &'g');

        assert_eq!(result.unwrap(), CoOrd { i: 2, j: 3 });
    }
}
