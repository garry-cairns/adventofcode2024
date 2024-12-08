mod seventh;
mod utils;

fn main() {
    env_logger::init();
    let input = utils::file_input("./src/input.txt").unwrap();
    let result = seventh::concat_add_mult(&input);
    println!("{result:?}");
}
