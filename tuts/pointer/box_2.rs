/*
    Box pointer is to wrap the data in the heap.
    This is to help the compiler to know the type is of fixed size, not infinite.

    So, in this eg, the Box is used for recursive data structures.
    Source: https://www.youtube.com/watch?v=m76sRj2VgGo
*/

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

pub fn run() {
    // simple case where the Box is not needed
    let b = Box::new(5);
    println!("b = {}", b);

    // recursive case where the Box is needed
    // define a list:
    // [1, •]->[2, •]–>[3, nil]
    let l = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("l = {:?}", l);
}
