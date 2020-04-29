use list11_12;

mod common; // module declaration

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, list11_12::add_two(2));
}
