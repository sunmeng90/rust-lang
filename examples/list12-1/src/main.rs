#![allow(unused_variables)]

use std::env;

// cargo run --bin list12-1 abc efg
fn main() {
    // collect turn the iterator to vector
    let args: Vec<String> = env::args().collect(); //Collecting the command line arguments into a vector
    let query = &args[1];
    let filename = &args[2];

    println!("Search for {}", query);
    println!("In file {}", filename);

    println!("{:?}", args);
}
