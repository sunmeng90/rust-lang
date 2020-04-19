use std::io;
use std::io::Read;
use std::fs::File;

fn main() {
    let s = read_username_from_file().expect("Can't read file content");
    println!("Got content: {:?}", s);
}


fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?; // ? will call from function to convert error to the return error type and report it to caller
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}