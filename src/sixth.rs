#[path = "utils.rs"]
mod utils;
use ndarray::Array2;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, PartialEq)]
enum Direction {
    Up,    // (-1, 0)
    Right, // (0, 1)
    Down,  // (-1, 0)
    Left,  // (0, -1)
}

pub fn guard_path(input: &str) -> u32 {
    let mut grid = utils::vec_to_array2(utils::string_to_2d_array(input, utils::just_chars));
    let (visited, _) = find_path(&mut grid, None);
    (visited.len()) as u32
}

pub fn cycles(input: &str) -> u32 {
    let mut grid = utils::vec_to_array2(utils::string_to_2d_array(input, utils::just_chars));
    let (mut visited, _) = find_path(&mut grid, None);
    visited.remove(&find_guard(&grid));
    let mut cycle_coords: HashSet<utils::CoOrd> = HashSet::new();
    for location in visited {
        let mut new_grid = grid.clone();
        let (_, c) = find_path(&mut new_grid, Some(location));
        if c {
            cycle_coords.insert(location);
        }
    }
    cycle_coords.len() as u32
}

fn find_path(
    grid: &mut Array2<char>,
    new_obstacle: Option<utils::CoOrd>,
) -> (HashSet<utils::CoOrd>, bool) {
    let mut visited: HashSet<utils::CoOrd> = HashSet::new();
    let mut directions = vec![
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ]
    .into_iter()
    .cycle();
    let mut cursor = find_guard(&grid);
    visited.insert(cursor);
    match new_obstacle {
        Some(coord) => grid[[coord.i, coord.j]] = '#',
        None => {}
    }
    let (obstacles_by_row, obstacles_by_column) = utils::locate_all_in_grid(&grid, &'#');
    // The point here is to avoid hitting all the squares
    // We know where the guard is and whether it's travelling up, down, left or right
    // Since we know where the obstables are we can add any squares not already
    // in the visited set between the guard and the next obstacle immediately,
    // and then change direction and repeat.
    let mut ended = false;
    while !ended {
        let (newly_visited, termination, new_cursor) = walk(
            grid,
            &cursor,
            &directions.next().unwrap(),
            &obstacles_by_row,
            &obstacles_by_column,
        );
        if termination == '!' {
            visited.extend(newly_visited);
            ended = true;
        } else if termination == 'O' {
            ended = true;
            return (visited, true);
        } else {
            // cursor is now the last co-ord visited in the last run
            // no need to update direction as our cycle will do that
            cursor = new_cursor;
            // update our visited set
            visited.extend(newly_visited);
        }
    }
    (visited, false)
}

fn walk(
    grid: &mut Array2<char>,
    cursor: &utils::CoOrd,
    direction: &Direction,
    obstacles_by_row: &HashMap<usize, Vec<utils::CoOrd>>,
    obstacles_by_column: &HashMap<usize, Vec<utils::CoOrd>>,
) -> (Vec<utils::CoOrd>, char, utils::CoOrd) {
    let (height, width) = (grid.shape()[0], grid.shape()[1]);
    let default_vec: Vec<utils::CoOrd> = Vec::new();
    if direction == &Direction::Up {
        let obstacles = obstacles_by_column.get(&cursor.j).unwrap_or(&default_vec);
        let max_less_than_i = obstacles.iter().filter(|&x| x.i < cursor.i).max();

        match max_less_than_i {
            Some(value) => {
                let new_cursor = utils::CoOrd {
                    i: value.i + 1,
                    j: value.j,
                };
                if grid[[new_cursor.i, new_cursor.j]] == '⌜' {
                    println!("{:?}", new_cursor);
                    println!("{:?}", grid);
                    return (points_between(cursor, &new_cursor), 'O', new_cursor);
                } else {
                    grid[[new_cursor.i, new_cursor.j]] = '⌜';
                }
                return (points_between(cursor, &new_cursor), '#', new_cursor);
            }
            None => {
                return (
                    points_between(cursor, &utils::CoOrd { i: 0, j: cursor.j }),
                    '!',
                    *cursor,
                )
            }
        }
    } else if direction == &Direction::Down {
        let obstacles = obstacles_by_column.get(&cursor.j).unwrap_or(&default_vec);
        let min_greater_than_i = obstacles.iter().filter(|&x| x.i > cursor.i).min();

        match min_greater_than_i {
            Some(value) => {
                let new_cursor = utils::CoOrd {
                    i: value.i - 1,
                    j: value.j,
                };
                if grid[[new_cursor.i, new_cursor.j]] == '⌟' {
                    println!("{:?}", new_cursor);
                    println!("{:?}", grid);
                    return (points_between(cursor, &new_cursor), 'O', new_cursor);
                } else {
                    grid[[new_cursor.i, new_cursor.j]] = '⌟';
                }
                return (points_between(cursor, &new_cursor), '#', new_cursor);
            }
            None => {
                return (
                    points_between(
                        cursor,
                        &utils::CoOrd {
                            i: height - 1,
                            j: cursor.j,
                        },
                    ),
                    '!',
                    *cursor,
                )
            }
        }
    } else if direction == &Direction::Left {
        let obstacles = obstacles_by_row.get(&cursor.i).unwrap_or(&default_vec);
        let max_less_than_j = obstacles.iter().filter(|&x| x.j < cursor.j).max();

        match max_less_than_j {
            Some(value) => {
                let new_cursor = utils::CoOrd {
                    i: value.i,
                    j: value.j + 1,
                };
                if grid[[new_cursor.i, new_cursor.j]] == '⌞' {
                    println!("{:?}", new_cursor);
                    println!("{:?}", grid);
                    return (points_between(cursor, &new_cursor), 'O', new_cursor);
                } else {
                    grid[[new_cursor.i, new_cursor.j]] = '⌞';
                }
                return (points_between(cursor, &new_cursor), '#', new_cursor);
            }
            None => {
                return (
                    points_between(cursor, &utils::CoOrd { i: cursor.i, j: 0 }),
                    '!',
                    *cursor,
                )
            }
        }
    } else if direction == &Direction::Right {
        let obstacles = obstacles_by_row.get(&cursor.i).unwrap_or(&default_vec);
        let min_greater_than_j = obstacles.iter().filter(|&x| x.j > cursor.j).min();

        match min_greater_than_j {
            Some(value) => {
                let new_cursor = utils::CoOrd {
                    i: value.i,
                    j: value.j - 1,
                };
                if grid[[new_cursor.i, new_cursor.j]] == '⌝' {
                    println!("{:?}", new_cursor);
                    println!("{:?}", grid);
                    return (points_between(cursor, &new_cursor), 'O', new_cursor);
                } else {
                    grid[[new_cursor.i, new_cursor.j]] = '⌝';
                }
                return (points_between(cursor, &new_cursor), '#', new_cursor);
            }
            None => {
                return (
                    points_between(
                        cursor,
                        &utils::CoOrd {
                            i: cursor.i,
                            j: width - 1,
                        },
                    ),
                    '!',
                    *cursor,
                )
            }
        }
    } else {
        (vec![], '!', *cursor)
    }
}

// Returns a vector of struct CoOrd { i: usize, j: usize } going in a straight line
// from start to end, in the order you'd expect from those two values
// Only works in straight lines.
fn points_between(start: &utils::CoOrd, end: &utils::CoOrd) -> Vec<utils::CoOrd> {
    let mut result = Vec::new();
    let static_i = start.i == end.i;
    let static_j = start.j == end.j;

    if static_i {
        let (min_j, max_j) = (min(start.j, end.j), max(start.j, end.j));
        for j in min_j..=max_j {
            result.push(utils::CoOrd { i: start.i, j });
        }
    } else if static_j {
        let (min_i, max_i) = (min(start.i, end.i), max(start.i, end.i));
        for i in min_i..=max_i {
            result.push(utils::CoOrd { i, j: start.j });
        }
    }
    result
}

fn find_guard(grid: &Array2<char>) -> utils::CoOrd {
    let chars: Vec<char> = vec!['^', '>', '∨', '<'];
    for guard in chars {
        let possible = utils::locate_in_grid(grid, &guard);
        if possible.is_some() {
            return possible.unwrap();
        }
    }
    return utils::CoOrd { i: 0, j: 0 };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guard_path() {
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

        let result = guard_path(input);
        assert_eq!(result, 41);
    }

    #[test]
    fn test_cycles() {
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

        let result = cycles(input);
        assert_eq!(result, 7);
    }
}
