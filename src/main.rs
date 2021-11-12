//CLI to view todo list

mod utils;

use std::env;
use std::collections::HashMap;

use configparser::ini::Ini;

fn main() {
    let mut config = Ini::new();
    let map = config.load("todo.conf")
        .expect("Unable to open file!");

    let list_ids = map.get("lists")
        .expect("No lists found!");

    let home_ids = map.get("home");
    
    let args: Vec<String> = env::args().collect();
    match args.get(1) {
        Some(letter) => match letter.as_ref() {
            "l" => list(&list_ids, &home_ids),              //ordinary list
            "u" => update(&list_ids),                       //update records
            "i" => interactive(&list_ids),                  //interactive
            _ => help("Invalid argument specified...")      //invalid option
        },
        None => help("No argument specified...")
    }
}

fn help(error_message: &str) {
    eprintln!("{}", error_message);

    println!("Usage: todo <mode>");
    println!("\nModes");
    println!("l - List Mode");
    println!("i - Interactive Mode");
    println!("u - Update list with AWS S3");
}

fn list(list_ids: &HashMap<String, Option<String>>, home_ids: &Option<&HashMap<String, Option<String>>>) {
    let display = |UUID: &String| {
        println!("{}", list_ids.get(UUID).unwrap().as_ref().unwrap());
        println!("----------");
        for (idx, item) in utils::read(&UUID).iter().enumerate() {
            if item.status == "TODO" {
                println!("{}. {}", idx, item.description)
            }
        }
    };

    match home_ids {
        Some(UUIDs) => {
            for UUID in UUIDs.keys() {
                display(UUID);
            }
        },
        None => {
            for UUID in list_ids.keys() {
                display(UUID)
            }
        }
    }
}

fn update(list_ids: &HashMap<String, Option<String>>) {
    println!("Updating list...");
    match utils::pull(&list_ids) {
        Ok(_) => println!("Update finished"),
        Err(_) => eprintln!("Error fetching updates")
    }
}

fn interactive(list_ids: &HashMap<String, Option<String>>) {
    println!("interactive")
}