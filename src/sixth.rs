#[path = "utils.rs"]
mod utils;
use std::collections::HashSet;

#[derive(Debug, Default)]
struct Guard {
    location: utils::CoOrd,
    orientation: char,
}

#[derive(Debug, Default)]
struct GameState {
    grid: Vec<Vec<char>>,
    guard: Guard,
}

pub fn guard_path(input: &str) -> u32 {
    let grid = utils::string_to_2d_array(input, utils::just_chars);
    let cursor = find_guard(&grid);
    let path: Vec<utils::CoOrd> = Vec::new();
    walk(&grid, &cursor, &path)
        .iter()
        .collect::<HashSet<_>>()
        .len() as u32
}

pub fn cycles(input: &str) -> u32 {
    3
}

fn walk(
    grid: &Vec<Vec<char>>,
    cursor: &utils::CoOrd,
    path: &Vec<utils::CoOrd>,
) -> Vec<utils::CoOrd> {
    let mut new_path: Vec<utils::CoOrd> = path.iter().copied().collect();
    new_path.push(cursor.clone());
    let proposed_move = next(grid, cursor);
    match proposed_move {
        Some(coord) => {
            let updated = update_grid(grid, &coord);
            new_path.extend(walk(&updated.grid, &updated.guard.location, &new_path));
        }
        None => println!("Stopped"),
    }
    new_path
}

fn update_grid(grid: &Vec<Vec<char>>, proposed: &utils::CoOrd) -> GameState {
    let mut new_grid = grid.clone();
    let guard_at_start = find_guard(&new_grid);
    let mut guard_at_end: Guard = Guard::default();

    if grid[proposed.i][proposed.j] == '#' {
        guard_at_end.location = guard_at_start;
        guard_at_end.orientation = turn(&new_grid[guard_at_start.i][guard_at_start.j]);
        new_grid[guard_at_start.i][guard_at_start.j] = guard_at_end.orientation;
    } else {
        new_grid[proposed.i][proposed.j] = new_grid[guard_at_start.i][guard_at_start.j];
        guard_at_end.location = proposed.clone();
        new_grid[guard_at_start.i][guard_at_start.j] = 'X';
    }

    GameState {
        grid: new_grid,
        guard: guard_at_end,
    }
}

fn find_guard(grid: &Vec<Vec<char>>) -> utils::CoOrd {
    let chars: Vec<char> = vec!['^', '>', '∨', '<'];
    for guard in chars {
        let possible = utils::locate_in_grid(grid, &guard);
        if possible.is_some() {
            return possible.unwrap();
        }
    }
    return utils::CoOrd { i: 0, j: 0 };
}

fn next(grid: &Vec<Vec<char>>, cursor: &utils::CoOrd) -> Option<utils::CoOrd> {
    let i_max = grid.len();
    let j_max = grid[0].len();
    if grid[cursor.i][cursor.j] == '^' {
        if cursor.i >= 1 {
            return Some(utils::CoOrd {
                i: cursor.i - 1,
                j: cursor.j,
            });
        }
    } else if grid[cursor.i][cursor.j] == '>' {
        if cursor.j + 1 < j_max {
            return Some(utils::CoOrd {
                i: cursor.i,
                j: cursor.j + 1,
            });
        }
    } else if grid[cursor.i][cursor.j] == '∨' {
        if cursor.i + 1 < i_max {
            return Some(utils::CoOrd {
                i: cursor.i + 1,
                j: cursor.j,
            });
        }
    } else {
        if cursor.j >= 1 {
            return Some(utils::CoOrd {
                i: cursor.i,
                j: cursor.j - 1,
            });
        }
    }
    return None;
}

fn turn(guard: &char) -> char {
    if guard == &'^' {
        '>'
    } else if guard == &'>' {
        '∨'
    } else if guard == &'∨' {
        '<'
    } else {
        '^'
    }
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
        assert_eq!(result, 6);
    }
}