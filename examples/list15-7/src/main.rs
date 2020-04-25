fn main() {
    let x = 5;
    let y = Box::new(x); // use smart pointer Box
    assert_eq!(5, x);
    assert_eq!(5, *y); // the deference operator * will follow the box's pointer to get the actual value 5
}

// fn main() {
//     let x = 5;
//     let y = &x;

//     assert_eq!(5, x);
//     assert_eq!(5, *y); // use * to deference a normal pointer(reference)
// }
