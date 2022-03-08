/* 
    Learn Generics
    - x, y are of same type T
    - function output as type T
    - for addition as operation, use `std::ops::Add<Output>` type for T

    SOURCE: https://learning-rust.github.io/docs/b4.generics.html
*/

fn add<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
    x + y
}

pub fn run() {
    let a = 2;
    let b = 4;

    println!("{}", add(a, b));
}