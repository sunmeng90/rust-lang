fn main() {
    println!("======================match_literals======================================");
    match_literals();
    println!("======================match_named_variables===============================");
    match_named_variables();
    println!("======================match_multiple_patterns=============================");
    match_multiple_patterns();
    println!("======================match_ranges_of_numeric_values======================");
    match_ranges_of_numeric_values();
    println!("======================match_ranges_of_char_values=========================");
    match_ranges_of_char_values();
}

fn match_literals() {
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn match_named_variables() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // Some(5) => println!("Got 5"), // Got 5, 5 matched the value in Some
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}

fn match_multiple_patterns() {
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn match_ranges_of_numeric_values() {
    let x = 5;
    match x {
        1..=5 => println!("one through five"), // range 1..=5
        _ => println!("something else"),
    }
}

fn match_ranges_of_char_values() {
    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}
