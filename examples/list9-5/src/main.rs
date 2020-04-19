#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(f) => f,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(err) => panic!("Problem creating the file: {:?}", err),
            },
            other_error => panic!("Problem opening the file {:?}", other_error),
        },
    };
    println!("Get file: {:?}", f);
}

fn get_or_create_file_using_closure() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        // the Result<T, E> type has many methods that accept a closure and are implemented using match expressions
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

// the unwrap method will call panic if there is any error
fn get_file_and_unwrap(){
    let f = File::open("hello.txt").unwrap(); // if succeed, then expression return the file, otherwise will call panic for us
}

// the expect method panic with a customized message
fn get_file_and_expect(){
    let f = File::open("hello.txt").expect("Failed to open hello.txt"); // if succeed, then expression return the file, otherwise will call panic for us with a custimized message
}