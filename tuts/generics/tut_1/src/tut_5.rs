/*
   Here, we implement methods for generic struct
*/

use std::ops::{Add, Mul};

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// Here, the Point is of generic type
impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
    fn get_y(&self) -> &T {
        &self.y
    }

    /*
    NOTE: we can attempt this as well, but we have to add required trait bounds in order to satisfy
    using functions like `sqrt()`, `pow(i)`
     */
    // fn distance_from_origin(&self) -> f32 {
    //     (self.x.powi(2) + self.y.powi(2)).sqrt()
    // }
}

// Here, the Point is of type f32
// NOTE: Sometimes easy to implement this rather than putting inside generic implementation.
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

pub fn main() {
    // able to print point-p1 with the impl methods
    // Here, we we need to cast the values as `f32`
    let p1 = Point {
        x: 1 as f32,
        y: 2 as f32,
    };
    println!("Point p1: ({0}, {1})", p1.get_x(), p1.get_y());
    // for this we need to
    println!("Distance from origin: {}", p1.distance_from_origin());

    // able to print point-p2 with the impl methods & `distance_from_origin()`
    let p2 = Point { x: 3.0, y: 4.0 };
    println!("Point p2: ({0}, {1})", p2.get_x(), p2.get_y());
    println!("Distance from origin: {}", p2.distance_from_origin());
}
