//CLI to view todo list

mod utils;

use std::env;
use std::collections::HashMap;

use configparser::ini::Ini;

fn main() {
    //Config stuff
    let mut config = Ini::new();
    let map = config.load("todo.conf")
        .expect("Unable to open file!");

    let list_ids = map.get("lists")
        .expect("No lists found!");

    let home_ids = map.get("home");
    
    //Parse arguments
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

/// Prints and error message and the available options in the event that the user gives an invalid parameter
/// 
/// We specifically use eprintln for the error message so that it will trigger stderr for the message.
/// The ordinary help information is written using the standard println
fn help(error_message: &str) {
    eprintln!("{}", error_message);

    println!("Usage: todo <mode>");
    println!("\nModes");
    println!("l - List Mode");
    println!("i - Interactive Mode");
    println!("u - Update list with AWS S3");
}

/// Outputs the tasks in a non-interactive format to the console
/// 
/// If there are lists specified in the [HOME] section of the conf file, then only those lists will be printed.
/// The default behavior is to print every list available in the [LISTS] section.
/// 
/// This function can be called if the user wants the tasks to be printed whenever they spawn a new terminal for example.
fn list(list_ids: &HashMap<String, Option<String>>, home_ids: &Option<&HashMap<String, Option<String>>>) {

    //Closure that does the actual printing
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
                println!("")
            }
        },
        None => {
            for UUID in list_ids.keys() {
                display(UUID);
                println!("")
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