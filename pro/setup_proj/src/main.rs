mod something {
    pub struct A {
        pub a: i32,
    }

    struct B {
        b: i32,
    }
}

use crate::something::A;

fn main() {
    let first = A { a: 67 };

    println!("{}", first.a);
}
