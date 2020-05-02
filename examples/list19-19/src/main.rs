fn main() {
    println!("A baby dog is called a {}", Dog::baby_name());
    // we want to call the baby_name method from the Animal trait as implemented on Dog
    // by saying that we want to treat the Dog type as an Animal for this function call
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}
