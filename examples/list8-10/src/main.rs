#![allow(unused_variables)]

fn main() {
    enum SpreadsheetCell {  // declar an enum that can hold different type value
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue"))
    ];

}
