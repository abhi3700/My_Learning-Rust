/* 
    Learn Generics
    - x, y are of same type T1
    - function output as type T2
    - for addition as operation, use `std::ops::Add<Output>` type for T

    Here, M-1 & M-2 are both correct. But, `try_into` seems much easy to implement in terms of no need to know the `from` type, whereas
    the `try_from` needs to know the `from` type.

    SOURCE: https://learning-rust.github.io/docs/b4.generics.html
*/

fn add<T1: std::ops::Add<Output = T2>, T2>(x: T1, y: T1) -> T2 {
    x + y
}

pub fn run() {
    let a: i32 = 2;
    let b: u32 = 4;

    println!("{}", add(a, b.try_into().unwrap()));      // M-1
    println!("{}", add(a, i32::try_from(b).unwrap()));  // M-2
}