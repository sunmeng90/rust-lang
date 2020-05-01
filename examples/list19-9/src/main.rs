// Defining and using an immutable static variable
static HELLO_WORLD: &str = "Hello, world!";

fn read_immutable_static_var() {
    println!("name is: {}", HELLO_WORLD);
}

// mutable static variable
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    // With mutable data that is globally accessible, it’s difficult to ensure there are no data races, which is why Rust considers mutable static variables to be unsafe.
    //
    unsafe {
        COUNTER += inc;
    }
}
// This code compiles and prints COUNTER: 3 as we would expect because it’s single threaded. 
// Having multiple threads access COUNTER would likely result in data races.
fn read_write_mutable_static_var() {
    add_to_count(3);

    // Accessing and modifying mutable static variables is unsafe.
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

fn main() {
    read_immutable_static_var();
    read_write_mutable_static_var();
}
