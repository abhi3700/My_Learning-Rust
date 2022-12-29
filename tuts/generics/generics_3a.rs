/*

    Apply generics over `struct`

    🔎 When adding an implementation for a generic struct, the type parameters should be declared after the impl as well
    impl<T> Point<T> {
*/

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

pub fn main() {
    let float = Point { x: 0.0, y: 5.6 };
    let integer = Point { x: 0, y: 5 };
    let boolean = Point { x: true, y: false };
    let char = Point { x: 'a', y: 'b' };
    let stringslice = Point {
        x: "Ram",
        y: "Sita",
    };

    println!("{:?}", float);
    println!("{:?}", integer);
    println!("{:?}", boolean);
    println!("{:?}", char);
    println!("{:?}", stringslice);
}
