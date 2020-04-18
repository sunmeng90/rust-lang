#![allow(unused_variables)]
#![allow(dead_code)]
fn main() {
    let v1: Vec<i32> = Vec::new(); // declear a vector that can hold a sequence of i32 values

    let v2 = vec![1, 2, 3]; // declear a vector and intialize with 3 i32 values, rust can infer the type of value for the vector
    {
        let mut v3 = Vec::new(); // declear a vector
        v3.push(1); // rust can infer the type of value is i32 for the vector
        v3.push(2);
        v3.push(3);
    } // v3 goes out of scope and is free here, all its elements are also dropped

    let v4 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v4[2]; // get a reference to the third element in the vector
    println!("The third element is {}", third);
    match v4.get(2) {
        Some(x) => println!("the third element is {}", x),
        None => println!("There is no third element"),
    };

    let does_not_exist = &v4[100]; // panicked at 'index out of bounds: the len is 5 but the index is 100'
    let does_not_exist = v4.get(100); // return None
}

fn list8_7() {
    // let mut v = vec![1, 2, 3];
    // let first = &v[0];
    // v.push(6); // may cause memory re-allocation
    // println!("The first element is {}", first); // maybe a invalid reference, when memory re-allocation happens
}

// error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
//  --> src/main.rs:6:5
//   |
// 4 |     let first = &v[0];
//   |                  - immutable borrow occurs here
// 5 |
// 6 |     v.push(6);
//   |     ^^^^^^^^^ mutable borrow occurs here
// 7 |
// 8 |     println!("The first element is: {}", first);
//   |                                          ----- immutable borrow later used here

fn list8_8() {
    let v = vec![1, 2, 3];
    for i in &v {
        println!("{}", i);
    }
}

fn list8_9() {
    let mut v = vec![1, 2, 3];
    for i in &mut v {
        *i += 50; // we have to use the dereference operator (*) to get to the value in i before we can use the += operator
    }
}
