#![allow(dead_code)]

use std::io;
use std::io::Read;
use std::fs;
use std::fs::File;

fn main() {
    let s = read_username_from_file().expect("Can't read file content");
    println!("Got content: {:?}", s);
}


fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

// use std function
fn read_username_from_file_using_std() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}