/* 
    'AsRef'
    - Used to do a cheap reference-to-reference conversion.
    - better than Borrow trait
    - unlike AsRef, Borrow is used like `impl` for any `T`
    - if any single attribute of struct is used, then use `AsRef` than `Borrow`

    In this example:
    ================
    By creating a generic function that takes an AsRef<str> we express that 
    we want to accept all references that can be converted to &str as an argument. 
    Since both String and &str implement AsRef<str> we can accept both as input argument.

    SOURCE: https://docs.rs/sp-std/latest/sp_std/convert/trait.AsRef.html
*/

fn is_hello<T: AsRef<str>>(s: T) {
    assert_eq!("hello", s.as_ref())
}

pub fn run() {
    // define as str
    let s = "hello";
    is_hello(s);

    // define as String
    let s = "hello".to_string();
    is_hello(s);
}