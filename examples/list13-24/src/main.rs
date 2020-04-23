#![allow(unused_variables)]

use std::env;
use std::process;

use list13_24::Config;
use list13_24::run;

// cargo run --color=always --package list13-24 --bin list13-24 frog .\examples\list13-24\poem.txt
fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // run(config); // an Err may return here, need to handle it
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
