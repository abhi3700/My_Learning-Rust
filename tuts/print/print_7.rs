/*
   This is a new way of printing variable with latest rust release
*/

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

pub fn main() {
    // print a var
    let x = String::from("Abhijit Roy");
    println!("{x}");

    // print a struct
    let p = Point { x: 1, y: 2 };
    println!("{p:?}");
}
