/*
    implement `Display` trait for a struct in order to print
*/

use std::fmt::Display;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}", self.x, self.y)
    }
}

pub fn main() {
    let p = Point { x: 3, y: 4 };
    println!("Point is {}", p);
}
