/* 
    - Use of `Borrow` traits
    - Immutably borrows from an owned value.
*/

use std::borrow::Borrow;

fn check<T: Borrow<str>>(s: T) {
    assert_eq!("Hello", s.borrow());
}

pub fn run() {
    let s = "Hello".to_string();
    check(s);

    let s1 = "Hello";
    check(s1);

}