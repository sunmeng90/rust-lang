use std::thread;
fn main() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        // we need move to transfer the ownership to the thread. we’re guaranteeing Rust that the main thread won’t use v anymore.
        println!("Here's the vector: {:?}", v);
    });
    // drop(v); // compile error, var v is moved to the closure in another thread
    handle.join().unwrap();
}
