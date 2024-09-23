/* 
    Learning:
    - How to use  dereference a reference to a reference
    - When `rr` is parsed, both Case-1 & Case-2 works fine. This is because: 
    - When `r` is parsed, only Case-1 works fine.
        + Rust’s comparison operators (things like == and >= etc) are smart enough to traverse a chain of references 
    until they reach a value, as long as both operands have the same type. This means in practice, you can have as 
    many references to references as needed, the “synctactical cost” stays the same as the compiler will figure it out for you!

    SOURCE: https://blog.thoughtram.io/references-in-rust/
*/

pub fn run() {
    let x = 10;
    let r = &x;
    let rr = &r;

    if is_ten(rr) {
        println!("Same");
    }
}

// Case-1
// Ideally, this is to be used when `r` is parsed inside.
// But, this also works when `rr` is parsed.
// fn is_ten(val: &i32) -> bool {
//     *val == 10
// }

// Case-2: 
// Ideally, this is to be used when `rr` is parsed inside.
fn is_ten(val: &&i32) -> bool {
    **val == 10
}