/*
   Generic struct
*/

//! not valid for single type Point struct variables
// struct Point<T> {
//     x: T,
//     y: T,
// }

//! valid for multi type Point struct variables
struct Point<T, U> {
    x: T,
    y: U,
}

pub fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1.0, y: 2.0 };
    let p3 = Point { x: 1, y: 2.0 };
}
