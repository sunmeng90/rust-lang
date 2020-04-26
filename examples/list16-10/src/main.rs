use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx); // create a new producer by clone
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap(); // use producer tx
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap(); // use producer tx1
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        // we’re treating rx as an iterator.
        println!("Got: {}", received);
    }
}
