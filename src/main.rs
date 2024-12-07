mod sixth;
mod utils;

fn main() {
    env_logger::init();
    let input = utils::file_input("./src/input.txt").unwrap();
    let result = sixth::guard_path(&input);
    println!("{result:?}");
}
