#![allow(unused_variables)]
fn main() {
    auto_drop_when_out_of_scope();
    println!("==============================");
    manual_drop_eariler();
}

// when CustomSmartPointer instances were out of scope, will call drop to release resources
// Variables are dropped in the reverse order of their creation, so d was dropped before c. This example gives you a visual guide to how the drop method works;
fn auto_drop_when_out_of_scope() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}

fn manual_drop_eariler() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointers created.");
    // c.drop(); // explicit destructor calls not allowed
    drop(c); // std::mem::drop to explicitly drop a value before it goes out of scope
    println!("CustomSmartPointer dropped before the end of main.");
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
