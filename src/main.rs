mod ninth;
mod utils;

fn main() {
    env_logger::init();
    let input = utils::file_input("./src/input.txt").unwrap();
    let result = ninth::checksum(&input);
    println!("{result:?}");
}
