use std::collections::HashMap;

fn main() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    println!("{:?}", map);
    // print!("{} -> {}", field_name, field_value); // compile error, the two values now owned by map
}
