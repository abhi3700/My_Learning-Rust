#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

pub fn main() {
    let p1 = Point { x: 3, y: 4 };
    let p2 = Point { x: 5, y: 2 };
    if p1 == p2 {
        println!("Yes");
    } else {
        println!("No")
    }
}
