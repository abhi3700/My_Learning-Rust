/*
    Learn Generics
    - x, y are of same type T
    - function output as type T
    - for addition as operation, use `std::ops::Add<Output>` type for T

    SOURCE: https://learning-rust.github.io/docs/b4.generics.html
*/
use std::ops::Add;
fn add<T: Add<Output = T>>(x: T, y: T) -> T {
    x + y
}

pub fn main() {
    let a = 2.0;
    let b = 4.0;

    println!("{}", add(a, b));
}
