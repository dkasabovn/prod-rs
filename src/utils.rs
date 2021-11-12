use std::fs::*;
use std::fs;
use std::io::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ListItem {
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

/// Grabs all list files in todo.conf from S3
pub fn pull(list_ids: &HashMap<String, Option<String>>) -> std::io::Result<()> {
    //URL Scheme: https://s3.amazonaws.com/dk.todors.dev/
    for list_id in list_ids.values() {
        let UUID = list_id.as_ref().unwrap();
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
    }

    Ok(())
}

/// Read from the .tamu file and store in vector
pub fn read(UUID: &String) -> Vec<ListItem> {
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