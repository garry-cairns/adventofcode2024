#[path = "utils.rs"]
mod utils;
use itertools::Itertools;
use ndarray::Array2;
use std::collections::{HashMap, HashSet};

pub fn detect_antinodes(input: &str) -> u32 {
    let (grid, coords) = clean(input);
    let mut antinodes: HashSet<utils::CoOrd> = HashSet::new();
    // These would seem to be the same
    let (height, _width) = (grid.shape()[0], grid.shape()[1]);
    // Feels like the algo here should be:
    // Find distinct chars in grid and make them a key in a hashmap: done
    // For each char list all combinations of co-ord pairs: done
    // Iterate over the keys: done
    // Iterate over the pairs for that key: done
    // Find the distance between the pair: done
    // Check whether a point in each direction at the same distance will be in bounds
    // +1 to the total for each one that is
    coords
        .values()
        .flat_map(|set| set.iter())
        .for_each(|value| {
            antinodes.extend(local_antinodes(value, height, true));
        });
    antinodes.len() as u32
}

pub fn detect_antinodes_greedy(input: &str) -> u32 {
    let (grid, coords) = clean(input);
    let mut antinodes: HashSet<utils::CoOrd> = HashSet::new();
    // These would seem to be the same
    let (height, _width) = (grid.shape()[0], grid.shape()[1]);
    // Feels like the algo here should be:
    // Find distinct chars in grid and make them a key in a hashmap: done
    // For each char list all combinations of co-ord pairs: done
    // Iterate over the keys: done
    // Iterate over the pairs for that key: done
    // Find the distance between the pair: done
    // Check whether a point in each direction at the same distance will be in bounds
    // +1 to the total for each one that is
    coords
        .values()
        .flat_map(|set| set.iter())
        .for_each(|value| {
            antinodes.extend(local_antinodes(value, height, false));
        });
    antinodes.len() as u32
}

fn clean(
    input: &str,
) -> (
    Array2<char>,
    HashMap<char, Vec<(utils::CoOrd, utils::CoOrd)>>,
) {
    let grid = utils::vec_to_array2(utils::string_to_2d_array(input, utils::just_chars));
    let mut antennas: HashSet<char> = input.chars().collect();
    println!("There are {:?}", &antennas.len());
    let mut coords: HashMap<char, Vec<(utils::CoOrd, utils::CoOrd)>> = HashMap::new();
    antennas.remove(&'.');
    antennas.remove(&'\n');
    for antenna in antennas {
        let (row_matches, column_matches) = utils::locate_all_in_grid(&grid, &antenna);
        let mut all_coords: HashSet<utils::CoOrd> = HashSet::new();
        for value in row_matches.values() {
            all_coords.extend(value.clone());
        }
        for value in column_matches.values() {
            all_coords.extend(value.clone());
        }
        coords.insert(
            antenna,
            all_coords
                .iter()
                .combinations(2)
                .filter(|pair| pair[0] != pair[1])
                .map(|pair| (pair[0].clone(), pair[1].clone()))
                .collect(),
        );
    }
    (grid, coords)
}

fn local_antinodes(
    pair: &(utils::CoOrd, utils::CoOrd),
    max: usize,
    restrict_freq: bool,
) -> Vec<utils::CoOrd> {
    let mut result: Vec<utils::CoOrd> = Vec::new();
    let mut line = utils::Line::new(pair.0, pair.1);

    if restrict_freq {
        let (back, front) = line.extend((max, max));
        if back.is_some() {
            result.push(back.unwrap());
        }
        if front.is_some() {
            result.push(front.unwrap());
        }
    } else {
        let (d_i, d_j) = utils::distance_between(pair.0, pair.1);
        let back = line.extend_back_greedy(d_i, d_j, (max, max));
        let forward = line.extend_forward_greedy(d_i, d_j, (max, max));
        result.extend(back);
        result.extend(forward);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_antinodes() {
        let input = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

        let result = detect_antinodes(input);
        assert_eq!(result, 14);
    }

    #[test]
    fn test_detect_antinodes_greedy() {
        let input = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

        let result = detect_antinodes_greedy(input);
        assert_eq!(result, 34);
    }
}

/*
......#....#
...#....0...
....#0....#.
..#....0....
....0....#..
.#....A.....
...#........
#......#....
........A...
.........A..
..........#.
..........#.
*/

/*
vv....v....v
.v.v....0...
..v.v0....v.
..vv...0....
....0....v..
.v...vA....v
...v..v.....
v....v.v....
..v.....A...
....v....A..
.v........v.
...v......vv
*/
