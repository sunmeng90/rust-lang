fn main() {}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // associated functions are associated with the struct.
    // They’re still functions, not methods, because they don’t have an instance of the struct to work with
    // usage: let sq = Rectangle::square(3)
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }

    // define method on Rectangle struct
    fn area(&self) -> u32 { // user &self reference here, because we don't want to take ownership
        self.width * self.width
    }
}

// one struct can have multiple impl block
impl Rectangle {
    fn can_hold(&self, another: &Rectangle) -> bool {
        self.width > another.width && self.height > another.height
    }
}