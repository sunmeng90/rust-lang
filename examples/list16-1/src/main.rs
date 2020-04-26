use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("hi, number {} from the spawn thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi, number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1))
    } // the main thread may exit early than the spawn finishes iteration

    // handle.join().unwrap(); // let main thread wait for spawn thread to finish
}
