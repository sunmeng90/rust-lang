#![allow(unused_variables)]

use std::env;

// cargo run --bin list12-5 abc poem.txt
fn main() {
    // collect turn the iterator to vector
    let args: Vec<String> = env::args().collect(); //Collecting the command line arguments into a vector
    
    let (query, filename) = parse_config(&args);

    println!("Search for {}", query);
    println!("In file {}", filename);

    println!("{:?}", args);
}


fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];
    (query, filename)
}