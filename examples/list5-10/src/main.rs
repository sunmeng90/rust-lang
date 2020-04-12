fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect detail: {:?}", rect);  // TODO: pass var to macro doesn't transfer ownership
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect)
    );
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}