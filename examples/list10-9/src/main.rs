#![allow(dead_code)]

fn main() {
    struct_generic_one();
    struct_generic_two_and_mix();
}

fn struct_generic_one() {
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        // <T> after impl is requirement to tell Rust that the <T> after PointA is a generic type not concrete type
        fn x(&self) -> &T {
            &self.x
        }
    }

    let p1 = Point { x: 1, y: 1 };
    println!("p1.x = {}", p1.x())
}

fn struct_generic_two_and_mix() {
    struct Point<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point<T, U> {
        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1 = Point { x: 1, y: 1 };
    let p2 = Point { x: "hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
