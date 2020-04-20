#![allow(unused_variables)]

use std::env;
use std::fs;

// cargo run --bin list12-7 abc .\examples\list12-7\poem.txt
fn main() {
    // collect turn the iterator to vector
    let args: Vec<String> = env::args().collect(); //Collecting the command line arguments into a vector
    let config = Config::new(&args);

    println!("Search for {}", config.query);
    println!("In file {}", config.filename);

    let content =
        fs::read_to_string(&config.filename).expect("Someting went wrong reading the file");
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        // constructor
        if args.len() < 3 {
            panic!("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Config { query, filename }
    }
}
