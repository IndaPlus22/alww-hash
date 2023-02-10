use hash_lib::DB;
use serde::Deserialize;
use std::{
    env,
    fs::{self, OpenOptions},
    io::{self, BufRead},
};
mod hash_lib;
mod traits;

fn main() {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("data.csv")
        .unwrap();
    let help_message = "Correct format: cargo run [argument] [parameter]\nExample:        cargo run    get       cerise \n\n- if \"cerise\" should exist in the database, return its corresponding key.\n\n";
    let mut db = DB::new();
    let args: Vec<String> = env::args().collect();
    let arg = &args[1];
    let mut param = &String::new();
    if arg.as_str() != "print" {
        if args.len() < 3 {
            panic!(
                "\n\nError: missing or invalid arguments\n\n{}",
                help_message
            );
        }
        param = &args[2];
    }
    // let arg = "get".to_owned();
    // let param = "OoOoOo";
    let file_path = "data.asv";

    db.load(&mut file);

    match arg.as_str() {
        "insert" | "add" => db.insert(param.to_owned()),
        "delete" => db.delete(param.to_owned()),
        "get" => {
            let thing = db.get(param.to_owned());
            if thing == None {
                println!("No match found!");
            } else {
                println!("{}", thing.unwrap());
            }
        }
        "print" => db.print(),
        _ => {
            panic!("Error: could not parse the argument\n\n{}", help_message);
        }
    }
    db.write()
}
