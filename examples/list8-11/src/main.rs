#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]

fn main() {
    let mut s = String::new(); //creates a new empty string called s, which we can then load data into.

    let data = "initial contents";

    let s = data.to_string(); // will create a String value from the string literal

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    slice_str();
}

fn list8_16() {
    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2); // push_str take a string slice as parameter, hence doesn't take the ownership of passing in parameter
    println!("s2 is {}", s2);
}

fn list8_17() {
    let mut s = String::from("lo");
    s.push('l'); // push a single character
}

fn list8_18() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
}

fn format_str() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
}

fn indexing_str_error() {
    let s1 = String::from("hello");
    // let h = s1[0]; // compile error, rust think it's meaningless to acces the first byte and may cause bugs

    let len = String::from("世界").len();
    let hello = "世界";
    // let answer = &hello[0]; // compile error, rust think it's meaningless to acces the first byte and may cause bugs
}

fn slice_str() {
    let hello = "世界";

    let s = &hello[0..6];

    // let s1 = &hello[0..1]; // will panic at runtime, if the slice cause broken characters
}

fn iterate_over_string() {
    for c in "世界".chars() {
        // iterate over char sequence for the string
        println!("{}", c);
    }

    for c in "世界".bytes() {
        // iterate over bytes sequence for the string
        println!("{}", c);
    }
}
