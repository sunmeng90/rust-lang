fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect detail: {:?}", rect);  // TODO: pass var to macro doesn't transfer ownership
    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );
}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // define method on Rectangle struct
    fn area(&self) -> u32 { // user &self reference here, because we don't want to take ownership
        self.width * self.width
    }
}