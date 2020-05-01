fn main() {
    ignore_with__();
    ignore_parts_of_a_value_with_nested__();
    ignore_parts_of_a_tuple();
    ignore_an_unused_variable_by_start_its_name_with__();
    ignore_an_unused_variable_in_some_option_by_start_its_name_with_();
    ignore_an_unused_variable__();
    ignore_remaining_with_two_dots();
    ignore_the_remaining_in_tuple();
    ignore_the_remaining_in_ambiguous_way_will_cause_compile_error();
}

fn ignore_with__() {
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }

    foo(3, 4);
}

// We can also use _ inside another pattern to ignore just part of a value, for example, when we want to
// test for only part of a value but have no use for the other parts in the corresponding code we want to run.
// Listing 18-18 shows code responsible for managing a setting’s value. The business requirements are that the
// user should not be allowed to overwrite an existing customization of a setting but can unset the setting and
// give it a value if it is currently unset.
fn ignore_parts_of_a_value_with_nested__() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        // In the first match arm, we don’t need to match on or use the values inside either Some variant,
        // but we do need to test for the case when setting_value and new_setting_value are the Some variant.
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);
}

// This code will print Some numbers: 2, 8, 32, and the values 4 and 16 will be ignored.
fn ignore_parts_of_a_tuple() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => println!("Some numbers: {}, {}, {}", first, third, fifth),
    }
}

// Starting a variable name with an underscore to avoid getting unused variable warnings
fn ignore_an_unused_variable_by_start_its_name_with__() {
    // Here we get a warning about not using the variable y, but we don’t get a warning about not using the
    // variable preceded by the underscore.
    let _x = 5;
    let y = 10;
}

// Note that there is a subtle difference between using only _ and using a name that starts with an
// underscore. The syntax _x still binds the value to the variable, whereas _ doesn’t bind at all.
fn ignore_an_unused_variable_in_some_option_by_start_its_name_with_() {
    let s = Some(String::from("Hello!"));
    // ignore the unused warning, but still bind the value
    if let Some(_s) = s {
        println!("found a string");
    }
    // An unused variable starting with an underscore still binds the value, which might take ownership of the value
    // println!("{:?}", s); // error: borrow of moved value: `s`
}

fn ignore_an_unused_variable__() {
    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        // _ ignore and not bind
        println!("found a string");
    }
    // s doesn’t get moved into _.
    println!("{:?}", s);
}

fn ignore_remaining_with_two_dots() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        // ignore y,z
        Point { x, .. } => println!("x is {}", x),
    }
}

fn ignore_the_remaining_in_tuple() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        // Matching only the first and last values in a tuple and ignoring all other values
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }
}

// However, using .. must be unambiguous. If it is unclear which values are intended for matching and which should be
// ignored, Rust will give us an error.
//
// It’s impossible for Rust to determine how many values in the tuple to ignore before matching a value with
// second and then how many further values to ignore thereafter. This code could mean that we want to ignore
// 2, bind second to 4, and then ignore 8, 16, and 32; or that we want to ignore 2 and 4, bind second to 8,
// and then ignore 16 and 32; and so forth. The variable name second doesn’t mean anything special to Rust,
// so we get a compiler error because using .. in two places like this is ambiguous.
fn ignore_the_remaining_in_ambiguous_way_will_cause_compile_error() {
    let numbers = (2, 4, 8, 16, 32);

    // match numbers {
    //     // An attempt to use .. in an ambiguous way
    //     (.., second, ..) => println!("Some numbers: {}", second), //
    // }
}
