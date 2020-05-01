fn main() {
    println!("====================================destructuring_structs_fields_into_vars");
    destructuring_structs_fields_into_vars();
    println!("==========================================================================");
    destructuring_structs_using_struct_field_shorthand();
    println!("========================destructuring_structs_using_struct_field_shorthand");
    destructuring_structs_and_match_literal_values_in_one_pattern();
    println!("=============destructuring_structs_and_match_literal_values_in_one_pattern");

    destructing_enum_variants();
    println!("=================================================destructing_enum_variants");
    destructing_nested_structs_enum();
    println!("===========================================destructing_nested_structs_enum");
    destructing_structs_and_tuples();
    println!("============================================destructing_structs_and_tuples");
}

//Destructuring Structs
struct Point {
    x: i32,
    y: i32,
}
fn destructuring_structs_fields_into_vars() {
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p; // destructuring fields to var
    assert_eq!(0, a);
    assert_eq!(7, b);
}

fn destructuring_structs_using_struct_field_shorthand() {
    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p; // destructuring fields to var named the same as fields
    assert_eq!(0, x);
    assert_eq!(7, y);
}

fn destructuring_structs_and_match_literal_values_in_one_pattern() {
    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis:({}, {})", x, y),
    }
}
//Destructing enums
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Destructuring enum variants that hold different kinds of values
fn destructing_enum_variants() {
    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => println!("The Quit variant has no data to destructure."),
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }
}

// Destructuring Nested Structs and Enums
fn destructing_nested_structs_enum() {
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color), // ChangeColor has value of Color type
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }
}

// Destructuring Structs and Tuples
fn destructing_structs_and_tuples() {
    struct Point {
        x: i32,
        y: i32,
    }
    // a complicated destructure where we nest structs and tuples inside a tuple and destructure all the primitive values out:
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
}
