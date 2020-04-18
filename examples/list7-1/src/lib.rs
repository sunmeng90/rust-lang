mod front_of_house {
    pub mod hosting {
        // make hosting module public
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// Modules can also hold definitions for other items, such as structs, enums, constants, traits, functions,
// or other modules
// src/main.rs and src/lib.rs are called crate roots. The reason for their name is that the contents
// of either of these two files form a module named crate at the root of the crate’s module structure,
// known as the module tree.
// =================================================================================================
// crate                       // entire module tree is rooted under the implicit module named crate.
//     └── front_of_house
//         ├── hosting
//         │   ├── add_to_waitlist
//         │   └── seat_at_table
//         └── serving
//             ├── take_order
//             ├── serve_order
//             └── take_payment

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist(); // `crate` is the root of crate's module tree

    // relative path
    self::front_of_house::hosting::add_to_waitlist();
}

//==================================================================================================
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order(); // access function in parent module using `super` keyword
    }

    fn cook_order() {}
}
