fn main() {
    println!("========================if let pattern==================");
    if_let_pattern();
    println!("========================while let pattern===============");
    while_let_pattern();
    println!("========================for let pattern=================");
    for_pattern();
    println!("========================let pattern=====================");
    let_pattern();
    println!("========================function parameter pattern======");
    fn_parameter_pattern();
}

fn if_let_pattern() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

fn while_let_pattern() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("{}", top)
    }
}

// The first call to enumerate produces the tuple (0, 'a'). When this value is matched to the pattern (index, value), 
// index will be 0 and value will be 'a', printing the first line of the output.
fn for_pattern() {
    let v = vec!['a', 'b', 'c'];
    for (idx, val) in v.iter().enumerate() {
        println!("{} is at index {}", val, idx);
    }
}

// let PATTERN = EXPRESSION;
fn let_pattern() {
    // In statements like let x = 5; with a variable name in the PATTERN slot, the variable name is just a particularly 
    // simple form of a pattern. Rust compares the expression against the pattern and assigns any names it finds. So in 
    // the let x = 5; example, x is a pattern that means “bind what matches here to the variable x.” Because the name x 
    // is the whole pattern, this pattern effectively means “bind everything to the variable x, whatever the value is.”
    let x = 5;
    println!("{}", x);
    // Here, we match a tuple against a pattern. Rust compares the value (1, 2, 3) to the pattern (x, y, z) and sees 
    // that the value matches the pattern, so Rust binds 1 to x, 2 to y, and 3 to z. You can think of this tuple pattern 
    // as nesting three individual variable patterns inside it.
    let (x, y, z) = (1, 2, 3);
    println!("{}, {}, {}", x, y, z);
    // If the number of elements in the pattern doesn’t match the number of elements in the tuple, the overall type won’t 
    // match and we’ll get a compiler error.
    // let (x, y) = (1, 2, 3); // error: expected a tuple with 3 elements, found one with 2 elements
}

fn fn_parameter_pattern() {
    fn foo(x: i32) {
        println!("parameter x: {}", x);
    }
    foo(10);

    fn bar(&(x, y): &(i32, i32)) {
        println!("parameter x: {}, y: {}", x, y);
    }
    bar(&(5, 10));
}
