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


pub fn run() {
    let point_a = Point {
        x: 0.0,
        y: 5.6
    };

    println!("{:?}", point_a);

    let point_b = Point {
        x: 0,
        y: 5
    };

    println!("{:?}", point_b);
}