// Using a semicolon after mod front_of_house rather than using a block tells Rust to load the
// contents of the module from another file with the same name as the module.
mod front_of_house; // Declaring the front_of_house module whose body will be in src/front_of_house.rs

pub use crate::front_of_house::hosting;


pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
