use list19_30_hello_macro::HelloMacro;
use list19_30_hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}
