// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// use crate::List::{Cons, Nil};

// fn main() {
//     let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
//     let b = Cons(3, Box::new(a)); // a is moved
//     let c = Cons(4, Box::new(a)); // can't be moved again
// }
#![allow(unused_variables)]

use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    references_count_increase();
    println!(
        "===================================================================================="
    );
    references_count_increase_decrease();
}

fn references_count_increase() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil))))); // Reference 1 -> a
                                                               // The call to Rc::clone only increments the reference count, which doesn’t take much time.
    let b = Cons(3, Rc::clone(&a)); // Rc::clone(&a) reference 2 -> a
    let c = Cons(4, Rc::clone(&a)); // Rc::clone(&a) reference 3 -> a
}

fn references_count_increase_decrease() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil))))); // Reference 1 -> a
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a)); // Rc::clone(&a) reference 2 -> a
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let c = Cons(4, Rc::clone(&a)); // Rc::clone(&a) reference 3 -> a
        println!("count after creating c = {}", Rc::strong_count(&a));
    } // c goes out of scope, reference decrease by 1
    println!("count after c goes out of scope = {}", Rc::strong_count(&a)); // references count = 2
}
