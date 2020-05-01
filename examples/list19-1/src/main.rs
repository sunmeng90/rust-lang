fn main() {
    create_raw_pointers_from_refs();
    // create_raw_pointers_from_arbitrary_mem_addr();
    create_raw_pointers_from_refs_and_deferencing_raw_pointers_within_unsafe_block();
}

// Notice that we don’t include the unsafe keyword in this code. We can create raw pointers in safe code;
// we just can’t dereference raw pointers outside an unsafe block, as you’ll see in a bit.
fn create_raw_pointers_from_refs() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
}

// Creating a raw pointer to an arbitrary memory address
// Trying to use arbitrary memory is undefined: there might be data at that address or there might not,
// the compiler might optimize the code so there is no memory access, or the program might error with a
// segmentation fault. Usually, there is no good reason to write code like this, but it is possible.
fn create_raw_pointers_from_arbitrary_mem_addr() {
    let address = 0x012345usize;
    let r = address as *const i32;
}


// Dereferencing raw pointers within an unsafe block
// Note also that in Listing 19-1 and 19-3, we created *const i32 and *mut i32 raw pointers that both 
// pointed to the same memory location, where num is stored. If we instead tried to create an immutable 
// and a mutable reference to num, the code would not have compiled because Rust’s ownership rules don’t 
// allow a mutable reference at the same time as any immutable references. With raw pointers, we can create 
// a mutable pointer and an immutable pointer to the same location and change data through the mutable 
// pointer, potentially creating a data race. Be careful!
fn create_raw_pointers_from_refs_and_deferencing_raw_pointers_within_unsafe_block() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    // we use the dereference operator * on a raw pointer that requires an unsafe block.
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}
