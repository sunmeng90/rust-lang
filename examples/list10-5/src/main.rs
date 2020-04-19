fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = larget(&number_list);
    println!("The largest number is {}", result);
}

fn larget<T>(list: &[T]) -> T {
    let mut larget = list[0];

    for &item in list.iter() {
        if item > larget { // compile error: binary operation `>` cannot be applied to type `T`
            larget = item;
        }
    }

    larget
}
