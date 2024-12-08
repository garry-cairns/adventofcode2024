mod eighth;
mod utils;

fn main() {
    env_logger::init();
    let input = utils::file_input("./src/input.txt").unwrap();
    let result = eighth::detect_antinodes_greedy(&input);
    println!("{result:?}");
}
