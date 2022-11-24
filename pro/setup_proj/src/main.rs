struct A {
    pub a: i32,
}

struct B {
    b: i32,
}

fn main() {
    let first = A { a: 67 };

    println!("{}", first.a);
}
