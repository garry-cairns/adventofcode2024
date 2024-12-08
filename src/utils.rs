use log::debug;
use ndarray::Array2;
use reqwest::blocking::get;
use reqwest::Error;
use std::collections::HashMap;
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

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CoOrd {
    pub i: usize,
    pub j: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Line {
    pub p1: CoOrd,
    pub p2: CoOrd,
}

impl Line {
    pub fn new(p1: CoOrd, p2: CoOrd) -> Self {
        Line { p1, p2 }
    }

    pub fn extend_back(&self, d_i: usize, d_j: usize, grid_size: (usize, usize)) -> Option<CoOrd> {
        let mut before_i = 0;
        if self.p1.i < self.p2.i {
            // Will put us over `max` if we're out of bounds
            before_i = self.p1.i.wrapping_sub(d_i);
        } else {
            before_i = self.p1.i + d_i;
        }
        let mut before_j = 0;
        if self.p1.j < self.p2.j {
            before_j = self.p1.j.wrapping_sub(d_j);
        } else {
            before_j = self.p1.j + d_j;
        }

        let before = if (before_i) >= 0
            && (before_j) >= 0
            && (before_i as usize) < grid_size.0
            && (before_j as usize) < grid_size.1
        {
            Some(CoOrd {
                i: before_i as usize,
                j: before_j as usize,
            })
        } else {
            None
        };
        before
    }

    pub fn extend_forward(
        &self,
        d_i: usize,
        d_j: usize,
        grid_size: (usize, usize),
    ) -> Option<CoOrd> {
        let mut after_i = 0;
        if self.p1.i < self.p2.i {
            // Will put us over `max` if we're out of bounds
            after_i = self.p2.i + d_i;
        } else {
            after_i = self.p2.i.wrapping_sub(d_i);
        }
        let mut after_j = 0;
        if self.p1.j < self.p2.j {
            after_j = self.p2.j + d_j;
        } else {
            after_j = self.p2.j.wrapping_sub(d_j);
        }

        let after = if (after_i) >= 0
            && (after_j) >= 0
            && (after_i as usize) < grid_size.0
            && (after_j as usize) < grid_size.1
        {
            Some(CoOrd {
                i: after_i as usize,
                j: after_j as usize,
            })
        } else {
            None
        };
        after
    }

    pub fn extend_back_greedy(
        &self,
        d_i: usize,
        d_j: usize,
        grid_size: (usize, usize),
    ) -> Vec<CoOrd> {
        let mut points: Vec<CoOrd> = Vec::new();
        let mut in_bounds = true;
        let mut local_d_i = d_i.clone();
        let mut local_d_j = d_j.clone();
        while in_bounds {
            let next_point = self.extend_back(local_d_i, local_d_j, grid_size);
            match next_point {
                Some(point) => points.push(point),
                None => in_bounds = false,
            }
            local_d_i += d_i;
            local_d_j += d_j;
        }
        points
    }

    pub fn extend_forward_greedy(
        &self,
        d_i: usize,
        d_j: usize,
        grid_size: (usize, usize),
    ) -> Vec<CoOrd> {
        let mut points: Vec<CoOrd> = Vec::new();
        points.push(self.p1);
        points.push(self.p2);
        let mut in_bounds = true;
        let mut local_d_i = d_i.clone();
        let mut local_d_j = d_j.clone();
        while in_bounds {
            let next_point = self.extend_forward(local_d_i, local_d_j, grid_size);
            match next_point {
                Some(point) => points.push(point),
                None => in_bounds = false,
            }
            local_d_i += d_i;
            local_d_j += d_j;
        }
        points
    }

    pub fn extend(&self, grid_size: (usize, usize)) -> (Option<CoOrd>, Option<CoOrd>) {
        let (d_i, d_j) = distance_between(self.p1, self.p2);
        let before = self.extend_back(d_i, d_j, grid_size);
        let after = self.extend_forward(d_i, d_j, grid_size);

        (before, after)
    }
}

//pub fn extend_until(line: &Line, grid_size: (usize, usize)) -> (Option<CoOrd>, Option<CoOrd>) {
//    let mut back: Vec<CoOrd> = Vec::new();
//    let mut front: Vec<CoOrd> = Vec::new();
//    let mut in_bounds = true;
//    while in_bounds {
//        let (back, front) = self.extend((max, max));
//        if back.is_some() {
//            back.push(back.unwrap());
//        }
//        if front.is_some() {
//            front.push(front.unwrap());
//        }
//        if back.is_none() && front.is_none {
//            in_bounds = true;
//        }
//    }
//}

pub fn distance_between(a: CoOrd, b: CoOrd) -> (usize, usize) {
    (
        (b.i as i32 - a.i as i32).abs() as usize,
        (b.j as i32 - a.j as i32).abs() as usize,
    )
}

// Assumes you're looking for all matches of `target`
pub fn locate_all_in_grid<T>(
    grid: &Array2<T>,
    target: &T,
) -> (HashMap<usize, Vec<CoOrd>>, HashMap<usize, Vec<CoOrd>>)
where
    T: PartialEq,
{
    let mut row_matches = HashMap::new();
    let mut col_matches = HashMap::new();

    grid.indexed_iter()
        .filter(|(_, value)| *value == target)
        .for_each(|((i, j), _)| {
            let coord = CoOrd { i, j };
            row_matches.entry(i).or_insert_with(Vec::new).push(coord);
            col_matches.entry(j).or_insert_with(Vec::new).push(coord);
        });

    (row_matches, col_matches)
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
    fn test_line() {
        let first = CoOrd { i: 2, j: 2 };
        let second = CoOrd { i: 3, j: 4 };
        let line = Line {
            p1: first,
            p2: second,
        };
        let (back, front) = line.extend((7, 7));
        if back.is_some() && front.is_some() {
            let expected = Line::new(back.unwrap(), front.unwrap());
            let e1 = CoOrd { i: 1, j: 0 };
            let e2 = CoOrd { i: 4, j: 6 };
            assert_eq!(expected.p1, e1);
            assert_eq!(expected.p2, e2);
        }
    }

    #[test]
    fn test_errant_line() {
        let first = CoOrd { i: 4, j: 4 };
        let second = CoOrd { i: 3, j: 7 };
        let line = Line {
            p1: first,
            p2: second,
        };
        let (back, front) = line.extend((7, 7));
        if back.is_some() && front.is_some() {
            let expected = Line::new(back.unwrap(), front.unwrap());
            let e1 = CoOrd { i: 2, j: 10 };
            let e2 = CoOrd { i: 5, j: 1 };
            assert_eq!(expected.p1, e1);
            assert_eq!(expected.p2, e2);
        }
    }

    #[test]
    fn test_distance_between() {
        let first = CoOrd { i: 1, j: 1 };
        let second = CoOrd { i: 2, j: 3 };
        let result = distance_between(first, second);
        assert_eq!(result, (1, 2));
        let reverse = distance_between(second, first);
        assert_eq!(reverse, (1, 2));
        let same = distance_between(first, first);
        assert_eq!(same, (0, 0));
    }

    #[test]
    fn test_find_all_in_grid() {
        let input = r#".....
.g...
...g.
....."#;
        let grid = vec_to_array2(string_to_2d_array(input, just_chars));
        let result = locate_all_in_grid(&grid, &'g');
        let mut expected_row = HashMap::new();
        expected_row.insert(1, vec![CoOrd { i: 1, j: 1 }]);
        expected_row.insert(2, vec![CoOrd { i: 2, j: 3 }]);
        let mut expected_column = HashMap::new();
        expected_column.insert(1, vec![CoOrd { i: 1, j: 1 }]);
        expected_column.insert(3, vec![CoOrd { i: 2, j: 3 }]);
        assert_eq!(result, (expected_row, expected_column));
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
