mod something;

use crate::something::*;

fn main() {
    let first = A { a: 67 };

    println!("{}", first.a);

    let second = B { b: 69 };

    println!("{}", second.b);
}
