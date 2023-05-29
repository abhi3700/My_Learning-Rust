//! Implement a `Add` trait for a `Point` struct.
//! The `Add` trait should add two points together by adding their `x` and `y` coordinates.

use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

/// manual implementation of `Add` trait
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub(crate) fn main() {
    let p1 = Point { x: 1, y: 0 };
    let p2 = Point { x: 2, y: 3 };

    let p3 = p1 + p2;

    assert_eq!(p3, Point { x: 3, y: 3 });
}
