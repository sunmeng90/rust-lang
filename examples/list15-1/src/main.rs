fn main() {
    let b = Box::new(5); // 5 will allocate on heap and Box pointer on stack will point to it
    println!("{:?}", b);
} // the box and value 5 will deallocated on stack and heap
