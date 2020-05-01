fn main() {
    at_operator_hold_value_and_test_match_against_a_pattern();
}

// Using @ lets us test a value and save it in a variable within one pattern.
fn at_operator_hold_value_and_test_match_against_a_pattern() {
    enum Message {
        Hello { id: i32 },
    }
    let msg = Message::Hello { id: 5 };
    match msg {
        // Using @ to bind to a value in a pattern while also testing it
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => println!("Found an id in another range"),
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}
