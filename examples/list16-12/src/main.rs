use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);
    {
        // lock returns a smart pointer called MutexGuard, wrapped in a LockResult that we handled with the call to unwrap.
        // The MutexGuard smart pointer implements Deref to point at our inner data; the smart pointer also has a Drop
        // implementation that releases the lock automatically when a MutexGuard goes out of scope
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m)
}
