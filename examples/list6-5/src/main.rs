#![allow(unused_variables)]

fn main() {
    // Rust use enum Option<T> over null to represent a value is absent or present
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            // Rust enum match is exhaustive, means that match arms need to cover every possible case
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let non = plus_one(None);
    // ==================================
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
        // The _ pattern will match any value, by putting it after other arms, the _ will
        // match all the possible cases that aren't specified before it. The () is just the unit value,
        // so nothing will happen in the _ case
    }
}

