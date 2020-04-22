fn main() {
    closure_capture_environment_value();
    closure_capture_environment_value_and_move();
}

fn closure_capture_environment_value() {
    let x = 4;
    let equal_to_x = |z| z == x; // x is not a parameter of the closure, but a variable from surrounding environment
    let y = 4;
    assert!(equal_to_x(y))
}


fn closure_capture_environment_value_and_move() {
    let x = vec![1, 2, 3];
    // If you want to force the closure to take ownership of the values it uses in the environment,
    // you can use the move keyword before the parameter list. This technique is mostly useful
    // when passing a closure to a new thread to move the data so it’s owned by the new thread.
    let equal_to_x = move |z| z == x; // move x ownership from enclosing context to the closure
    println!("can't use x here: {:?}", x);
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y))
}