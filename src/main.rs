mod third;
mod utils;

fn main() {
    env_logger::init();
    let input = utils::file_input("./src/input.txt").unwrap();
    let result = third::dodont(&input);
    println!("{result:?}");
}
