//! Rust By Practice
//! ================
//! https://practice.rs/generics-traits/generics.html#method-2
// Fix the errors to make the code work.
struct Point<T> {
    x: T,
    y: T,
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

pub fn main() {
    // DONE: casting done into `f32`
    let p = Point {
        x: 5 as f32,
        y: 10 as f32,
    };
    println!("{}", p.distance_from_origin());
}
