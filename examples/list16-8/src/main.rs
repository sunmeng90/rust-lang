use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap(); // send
        // println!("val is {}", val); // compile error, val is moved
    });

    let received = rx.recv().unwrap(); // receive
    println!("Got: {}", received);
}
