fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}

use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // uses self.0 to access the inner Vec<T>, because Wrapper is a tuple struct and Vec<T> is the item at index 0 in the tuple
        write!(f, "[{}]", self.0.join(", "))
    }
}
