fn main() {
    call_unsafe_function_or_method();
    use_the_safesplit_at_mut();
    
}

// We must call the dangerous function within a separate unsafe block. If we try to call
// dangerous without the unsafe block, we’ll get an error:
fn call_unsafe_function_or_method() {
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }
}

fn use_the_safesplit_at_mut() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

// fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = slice.len();

//     assert!(mid <= len);
//     //we’re borrowing from the same slice twice
//     (&mut slice[..mid], &mut slice[mid..]) // error: cannot borrow `*slice` as mutable more than once at a time
// }

// fn main() {
//     let mut vector = vec![1, 2, 3, 4, 5, 6];
//     let (left, right) = split_at_mut(&mut vector, 3);
// }

// Note that we don’t need to mark the resulting split_at_mut function as unsafe, and we can call this function
// from safe Rust. We’ve created a safe abstraction to the unsafe code with an implementation of the function that
// uses unsafe code in a safe way, because it creates only valid pointers from the data this function has access to.
use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr(); // returns a raw pointer

    assert!(mid <= len);

    // Using unsafe code in the implementation of the split_at_mut function
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid), // unsafe
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

#[test]
fn split_at_mut_test() {
    let mut vector = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut vector, 3);
}


extern "C" {
    fn abs(input: i32) -> i32;
}

fn use_extern_to_call_external_code() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}


// use extern to create an interface that allows other languages to call Rust functions. 
// Instead of an extern block, we add the extern keyword and specify the ABI to use just 
// before the fn keyword. We also need to add a #[no_mangle] annotation to tell the Rust 
// compiler not to mangle the name of this function. 
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}