fn main() {
    let m1 = Millimeters(1);
    let m2 = Meters(1);
    assert_eq!(m1 + m2, Millimeters(1001),)
}

use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
