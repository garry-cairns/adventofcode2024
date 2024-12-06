mod fifth;
mod utils;

fn main() {
    env_logger::init();
    let input = utils::file_input("./src/input.txt").unwrap();
    let result = fifth::corrected_total(&input);
    println!("{result:?}");
}
