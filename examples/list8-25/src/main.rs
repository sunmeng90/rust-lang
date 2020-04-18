#![allow(unused_variables)]
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50); // insert conditionaly
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}
