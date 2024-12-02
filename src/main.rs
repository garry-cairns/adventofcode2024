mod first;
mod second;
mod utils;

fn main() {
    env_logger::init();
    let input = utils::file_input("./src/input.txt").unwrap();
    let result = second::check_safety_dampened(&input);
    println!("{result:?}");
}
