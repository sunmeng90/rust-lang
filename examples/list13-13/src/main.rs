#![allow(dead_code)]
#![allow(unused_variables)]
fn main() {
    create_an_iterator();
    using_an_iterator_in_for_loop();
    iterator_demonstration();
    method_that_cosumes_iterators();
    method_that_produces_iterator();
}

// create but not consume it
fn create_an_iterator() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter(); // iterator created, but not consumed yet
}

// create and cosume it in a for loop
fn using_an_iterator_in_for_loop() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter(); // iterator created, but not consumed yet
    for val in v1_iter {
        println!("{:?}", val);
    }
}

// demonstrate how an iterator consumes elements
fn iterator_demonstration() {
    // Note that we needed to make v1_iter mutable: calling the next method on an iterator changes internal state that
    // the iterator uses to keep track of where it is in the sequence. In other words, this code consumes, or uses up,
    // the iterator. Each call to next eats up an item from the iterator. We didn’t need to make v1_iter mutable when
    // we used a for loop because the loop took ownership of v1_iter and made it mutable behind the scenes.

    // Also note that the values we get from the calls to next are immutable references to the values in the vector.
    // The iter method produces an iterator over immutable references. If we want to create an iterator that takes
    // ownership of v1 and returns owned values, we can call into_iter instead of iter. Similarly, if we want to
    // iterate over mutable references, we can call iter_mut instead of iter.
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    // next return immutable reference
    assert_eq!(v1_iter.next(), Some(&1)); // TODO: a reference to literal value really???, how can we create a reference to a literal value
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

fn method_that_cosumes_iterators() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
    // We aren’t allowed to use v1_iter after the call to sum because sum takes ownership of the iterator we call it on.
}

fn method_that_produces_iterator() {
    let v1 = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
}

// Using Closures that Capture Their Environment

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // we call into_iter to create an iterator that takes ownership of the vector. Then we call filter to adapt that 
    // iterator into a new iterator that only contains elements for which the closure returns true.
    // The closure captures the shoe_size parameter from the environment 
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[test]
fn filter_by_shoe_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            },
        ]
    );
}
