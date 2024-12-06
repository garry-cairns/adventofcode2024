#[path = "utils.rs"]
mod utils;
use std::collections::HashSet;

pub fn wordsearch(input: &str) -> i32 {
    // backtracking algorithm
    let board = utils::string_to_2d_array(input, utils::just_chars);
    let n = board.len();
    let word = "XMAS";
    let mut count = 0;

    let directions = vec![
        (0, 1),   // right
        (0, -1),  // left
        (1, 0),   // down
        (-1, 0),  // up
        (1, 1),   // down-right
        (1, -1),  // down-left
        (-1, 1),  // up-right
        (-1, -1), // up-left
    ];

    for i in 0..n {
        for j in 0..n {
            for &(dx, dy) in &directions {
                if search(&board, word, i, j, dx, dy, 0) {
                    count += 1;
                }
            }
        }
    }

    count
}

pub fn crosssearch(input: &str) -> i32 {
    let board = utils::string_to_2d_array(input, utils::just_chars);
    let n = board.len();
    let word = "A";
    let mut count = 0;

    let directions = vec![
        (1, 1),   // down-right
        (1, -1),  // down-left
        (-1, 1),  // up-right
        (-1, -1), // up-left
    ];

    // Starting by finding all the As to know where to centre searches
    // Starting from 1 in because an A at any edge won't be able to make a cross
    let centres = centre_matches(&board, &word, directions.clone());

    for centre in centres {
        let mut local_count = 0;
        let three_square = subarray(&board, &centre);

        for i in 0..n {
            for j in 0..n {
                for &(dx, dy) in &directions {
                    if search(&three_square, "MAS", i, j, dx, dy, 0) {
                        local_count += 1;
                    }
                }
            }
        }
        if local_count == 2 {
            count += 1;
        }
    }
    count
}

fn centre_matches(
    board: &Vec<Vec<char>>,
    word: &str,
    directions: Vec<(isize, isize)>,
) -> HashSet<(usize, usize)> {
    let n = board.len();
    let mut matches = HashSet::new();

    for i in 1..n - 1 {
        for j in 1..n - 1 {
            for &(dx, dy) in &directions {
                if search(&board, word, i, j, dx, dy, 0) {
                    matches.insert((i, j));
                }
            }
        }
    }
    matches
}

fn subarray(board: &Vec<Vec<char>>, centre: &(usize, usize)) -> Vec<Vec<char>> {
    let (centre_x, centre_y) = centre;
    (0..3)
        .map(|i| {
            (0..3)
                .map(|j| board[centre_x - 1 + i][centre_y - 1 + j])
                .collect()
        })
        .collect()
}

fn search(
    board: &Vec<Vec<char>>,
    word: &str,
    x: usize,
    y: usize,
    dx: isize,
    dy: isize,
    index: usize,
) -> bool {
    if index == word.len() {
        return true;
    }
    if x >= board.len() || y >= board.len() {
        return false;
    }
    if board[x][y] != word.chars().nth(index).unwrap() {
        return false;
    }

    search(
        board,
        word,
        (x as isize + dx) as usize,
        (y as isize + dy) as usize,
        dx,
        dy,
        index + 1,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wordsearch() {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

        let result = wordsearch(input);
        assert_eq!(result, 18);
    }

    #[test]
    fn test_crosssearch() {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

        let result = crosssearch(input);
        assert_eq!(result, 9);
    }
}
