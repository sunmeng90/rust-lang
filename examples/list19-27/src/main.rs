fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is {}", answer);
    let answer = do_twice_with_closure(return_closure(), 5);
    println!("Call closure, return: {}", answer);
    let answer = do_twice_with_closure(add_one, 5); // pass a function pointer as an argument for a function that expects a closure
    println!("Call function, The answer is {}", answer);
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// Function pointers implement all three of the closure traits (Fn, FnMut, and FnOnce), 
// so you can always pass a function pointer as an argument for a function that expects a closure. 
fn do_twice_with_closure<F>(f: F, arg: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(arg) + f(arg)
}

fn return_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
