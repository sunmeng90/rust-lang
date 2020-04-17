#![allow(unused_variables)]
fn main() {
    enum Message { // each variant can have different types and amounts of associated data.
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
}
