#![allow(unused_variables)]
#![allow(dead_code)]

fn main() {
    struct PointA<T> {
        x: T,
        y: T,
    }
    let p1 = PointA { x: 1, y: 1 };
    let p2 = PointA { x: 1.0, y: 1.0 };

    struct PointB<T, U> {
        x: T,
        y: U,
    }

    let p21 = PointB { x: 1, y: 1.0 };
    let p21 = PointB { x: 1, y: 1 };
}
