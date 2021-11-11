//CLI to view todo list

use std::fs::*;
use std::fs;
use std::io::prelude::*;

fn main() {
    println!("Hello, world!");

    let UUID = "a7a483d5-7425-40dc-8c11-0136768f72df".to_string();
    pull(&UUID);
    println!("{:#?}", read(&UUID))
}

/// Grabs a .csv file from Amazon S3
fn pull(UUID: &String) -> std::io::Result<()> {
    //URL Scheme: https://s3.amazonaws.com/dk.todors.dev/
    println!("https://s3.amazonaws.com/dk.todors.dev/{}.tamu", UUID);

    let response = match reqwest::blocking::get(format!("https://s3.amazonaws.com/dk.todors.dev/{}.tamu", UUID)) {
        Ok(v) => v.text(),
        Err(_) => panic!("Error making request to S3"),
    };

    let body = match response {
        Ok(v) => v,
        Err(_) => panic!("Error decoding to text"),
    };

    let mut file = File::create(format!("{}.tamu", UUID))?;
    file.write_all(body.as_bytes())?;

    Ok(())
}

#[derive(Debug)]
struct ListItem {
    status: String,
    description: String,
    ttl: i32,
}

impl ListItem {
    fn new(row: &str) -> ListItem {

        let fields: Vec<&str> = row.split(",").collect();
        let ttl: i32 = match fields[2].parse() {
            Ok(v) => v,
            Err(_) => panic!("File error! Unable to properly convert int!")
        };

        ListItem{
            status: fields[0].to_string(),
            description: fields[1].to_string(),
            ttl: ttl
        }
    }
}

/// Read from the .tamu file and store in vector
fn read(UUID: &String) -> Vec<ListItem> {
    let content =  fs::read_to_string(format!("{}.tamu", UUID))
        .expect("Error opening file!");

    let mut todo_list: Vec<ListItem> = Vec::new();

    for row in content.split("\n") {
        if row != "" {
            todo_list.push(ListItem::new(row))
        }
    }

    todo_list
}