/*
    Learn Generics
    - x, y are of same type T
    - function output as type T
    - for addition as operation, use `std::ops::Add<Output>` type for T

    SOURCE: https://learning-rust.github.io/docs/b4.generics.html
*/
fn sum<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

pub fn main() {
    assert_eq!(5, sum(2i8, 3i8));
    assert_eq!(50, sum(20, 30));
    assert_eq!(2.46, sum(1.23, 1.23));

    println!("Success!");
}
