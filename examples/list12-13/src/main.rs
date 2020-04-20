#![allow(unused_variables)]

use std::env;
use std::process;
use list12_13::Config;
use list12_13::run;

// cargo run --bin list12-12 abc .\examples\list12-12\poem.txt
fn main() {
    // collect turn the iterator to vector
    let args: Vec<String> = env::args().collect(); //Collecting the command line arguments into a vector
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // run(config); // an Err may return here, need to handle it
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
