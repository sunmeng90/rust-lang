#![allow(dead_code)]

use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    let s = read_username_from_file().expect("Can't read file content");
    println!("Got content: {:?}", s);
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(f) => f,
        Err(err) => return Err(err), // return error to the caller
    };

    let mut buf = String::new();

    match f.read_to_string(&mut buf) { // read bytes to buf and return the bytes number
        Ok(_) => Ok(buf), // we don't need the size, only return the string
        Err(err) => Err(err), // report error to the caller
    }
}
