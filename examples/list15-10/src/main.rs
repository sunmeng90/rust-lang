use std::ops::Deref;

fn main() {
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    // Our MyBox<T> type now implements Deref trait, now it can be dereferenced
    assert_eq!(5, *y); // myBox pointer is now dereferenced by *
    // ===============================================================================
    // Deref coercion with Functions and Methods
    // Deref coercion is a convenience that Rust performs on arguments to functions and methods. 
    // Deref coercion works only on types that implement the Deref trait. Deref coercion converts 
    // such a type into a reference to another type. 
    let msg = MyBox::new(String::from("Rust"));
    // hello function expects a &str as parameter
    // Calling hello with a reference to a MyBox<String> value, which works because of deref coercion

    // Here we’re calling the hello function with the argument &m, which is a reference 
    // to a MyBox<String> value. Because we implemented the Deref trait on MyBox<T>,
    // Rust can turn &MyBox<String> into &String by calling deref. The standard library provides an 
    // implementation of Deref on String that returns a string slice, and this is in the API documentation 
    // for Deref. Rust calls deref again to turn the &String into &str, which matches the hello function’s definition.
    hello(&msg);
}

struct MyBox<T>(T); // a tuple struct with one element

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// The Deref trait, provided by the standard library, requires us to implement one method named deref 
// that borrows self and returns a reference to the inner data.
impl<T> Deref for MyBox<T> {
    type Target = T; // an associated type

    fn deref(&self) -> &T {
        &self.0 // return a reference to the value, so we can access it with deference operator *
    }
}

// A hello function that has the parameter name of type &str
fn hello(name: &str) {
    println!("Hello, {}!", name);
}
