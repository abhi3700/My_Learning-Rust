#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

pub fn main() {
    let integer_and_boolean = Point { x: 5, y: false };
    let float_and_string = Point { x: 1.0, y: "hey" };
    let integer_and_float = Point { x: 5, y: 4.0 };
    let both_integer = Point { x: 10, y: 30 };
    let both_boolean = Point { x: true, y: true };
    println!("{:?}", integer_and_boolean);
}
