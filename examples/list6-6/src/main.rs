#![allow(unused_variables)]

fn main() {
    let some_u8_value = Some(0u8); // 0u8 is data type, not value
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (), // a lot of boilerplate code like this for ignore other case
    }
    // ================>
    // The if let syntax lets you combine if and let into a less verbose way to handle values that
    // match one pattern while ignoring the rest.
    if let Some(3) = some_u8_value {
        println!("three");
    }

    // if let can also has a else expression
    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!("NONE")
    }
}


