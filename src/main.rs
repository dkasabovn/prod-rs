//CLI to view todo list

mod utils;


use configparser::ini::Ini;

fn main() {
    println!("Hello, world!");

    let mut config = Ini::new();
    let map = config.load("todo.conf")
        .expect("Unable to open file!");

    let list_ids = map.get("lists")
        .expect("No lists found!");

    utils::pull(&list_ids);
}