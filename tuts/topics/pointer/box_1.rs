//! Box: A pointer type for heap allocation
//! - **Ownership**: Boxes provide ownership for this allocation, and drop their contents when they go out of scope.
//! - **Size limitation**: Boxes also ensure that they never allocate more than `isize::MAX` bytes.
//! - **Deref coercion**: Box implements the Deref trait, which allows it to be treated
//!     like a reference to the value it contains. This means that you can use Box
//!     values in the same way as other types of pointers, without having to explicitly dereference them.
//!
//! [`SUMMARY`]:
//! Overall, Box provides a safe and convenient way to allocate memory on the heap and manage ownership of
//! dynamically-sized data structures in Rust.
//!
//! SOURCE: https://docs.rs/sp-std/latest/sp_std/boxed/index.html

pub fn main() {
    let val: u8 = 5; // in stack
    let boxed_val: Box<u8> = Box::new(val); // in heap

    println!("boxed = {}", boxed_val); // 5
    println!("{:?}", *boxed_val); // 5

    let boxed2: Box<u8> = Box::new(78); // in heap
    let val2 = *boxed2; // Box by default implements `Deref` trait.

    println!("{:?}", val2); // 78
}
