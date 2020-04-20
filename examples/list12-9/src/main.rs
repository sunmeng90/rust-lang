#![allow(unused_variables)]

use std::env;
use std::fs;

// cargo run --bin list12-9 abc .\examples\list12-9\poem.txt
fn main() {
    // collect turn the iterator to vector
    let args: Vec<String> = env::args().collect(); //Collecting the command line arguments into a vector
    let config = match Config::new(&args) {
        Ok(config) => config,
        Err(err) => panic!(err)
    };

    let content =
        fs::read_to_string(&config.filename).expect("Someting went wrong reading the file");
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> { // return Result instead of panic
        // constructor
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}
