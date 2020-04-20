#![allow(unused_variables)]

use std::env;
use std::fs;

// cargo run --bin list12-6 abc .\examples\list12-6\poem.txt
fn main() {
    // collect turn the iterator to vector
    let args: Vec<String> = env::args().collect(); //Collecting the command line arguments into a vector
    let config = parse_config(&args);

    println!("Search for {}", config.query);
    println!("In file {}", config.filename);

    let content =
        fs::read_to_string(&config.filename).expect("Someting went wrong reading the file");
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();
    Config { query, filename }
}
