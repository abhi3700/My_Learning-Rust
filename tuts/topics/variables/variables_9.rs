/*
    Shadowing
*/

pub fn main() {
    let x = "THREE";
    println!("x: {}", x);
    // x = 3; // ERROR as it's not a mutable var, so we have 2 ways: shadowing (let) or mut x at L:6
    let x = 3;
    println!("x: {}", x + 2);
}
