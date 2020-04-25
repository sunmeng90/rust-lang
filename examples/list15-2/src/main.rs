#![allow(unused_variables)]

// At compile time, Rust needs to know how much space a type takes up. One type whose size 
// can’t be known at compile time is a recursive type, where a value can have as part of 
// itself another value of the same type. Because this nesting of values could theoretically 
// continue infinitely, Rust doesn’t know how much space a value of a recursive type needs. 
// However, boxes have a known size, so by inserting a box in a recursive type definition, 
// you can have recursive types.


// use crate::List::{Cons, Nil};
// fn main() {
//     let list = Cons(1, Cons(2, Cons(3, Nil)));
// }

// The compiler starts by looking at the Cons variant, which holds a value of type i32 and
// a value of type List. Therefore, Cons needs an amount of space equal to the size of an
// i32 plus the size of a List. To figure out how much memory the List type needs, the
// compiler looks at the variants, starting with the Cons variant. The Cons variant holds
// a value of type i32 and a value of type List, and this process continues infinitely

// enum List {
//     Cons(i32, List), // error: recursive type `List` has infinite size
//     Nil,
// }



// Using Box<T> to Point to Data on the Heap
// Boxes allow you to store data on the heap rather than the stack. What remains on the stack is the pointer to the heap data. 
enum List {
    Cons(i32, Box<List>), // Box is a smart pointer and the size is fixed
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
