#![allow(dead_code)]

fn main() {
    let x = 5;
    let y = MyBox(x);
    assert_eq!(5, x);
    // Our MyBox<T> type can’t be dereferenced because we haven’t implemented that ability on our type.
    // To enable dereferencing with the * operator, we implement the Deref trait.
    // assert_eq!(5, *y); // error: type `MyBox<{integer}>` cannot be dereferenced
}

struct MyBox<T>(T); // a tuple struct with one element

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
