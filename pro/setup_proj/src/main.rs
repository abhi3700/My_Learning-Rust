mod something {
    pub struct A {
        pub a: i32,
    }

    pub struct B {
        pub b: i32,
    }
}

use crate::something::*;

fn main() {
    let first = A { a: 67 };

    println!("{}", first.a);

    let second = B { b: 69 };

    println!("{}", second.b);
}
