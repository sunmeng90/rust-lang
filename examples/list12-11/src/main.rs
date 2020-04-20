#![allow(unused_variables)]

use std::env;
use std::fs;
use std::process;

// cargo run --bin list12-11 abc .\examples\list12-11\poem.txt
fn main() {
    // collect turn the iterator to vector
    let args: Vec<String> = env::args().collect(); //Collecting the command line arguments into a vector
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    run(config);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        // return Result instead of panic
        // constructor
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

fn run(config: Config) {
    let content = fs::read_to_string(config.filename).expect("Something went wrong reading the file");
    println!("with content: {}", content);
}