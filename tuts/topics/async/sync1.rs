//! This program demonstrates the use of `Rc` (Reference Counted) smart pointer in Rust.
//! `Rc` allows multiple ownership of the same data in a single thread.
//! The `Rc::clone` method increases the reference count of the `Rc` instance, which
//! increments the number of owners of the data. When the reference count becomes zero,
//! the data is dropped.
//!
//! This program creates an `Rc` instance of an integer value 42, and then creates two
//! references to the same data using `Rc::clone`. The program then prints the original
//! data and the two references.
//!
//! # Arguments
//!
//! This program does not accept any command line arguments.
//!
//! # Example
//!
//! ```
//! use std::rc::Rc;
//!
//! let data = Rc::new(42);
//! let reference_1 = Rc::clone(&data);
//! let reference_2 = Rc::clone(&data);
//!
//! assert_eq!(*data, 42);
//! assert_eq!(*reference_1, 42);
//! assert_eq!(*reference_2, 42);
//! ```
use std::rc::Rc;

pub(crate) fn main() {
    // create a data that can have shared ownership in single thread.
    let data = Rc::new(42);

    // create references
    let reference_1 = Rc::clone(&data);
    let reference_2 = Rc::clone(&data);

    // NOTE: here, * (asterisk) is NOT required to get the Rc pointer's data,
    // as `format!` automatically uses `Deref` trait method to do this:
    // Illustration: When you use an Rc<T> as an argument to a function
    // or macro that expects a reference to T, the Deref trait is automatically
    // called on the Rc<T>, which returns a reference to the value inside the Rc<T>.
    println!("original data: {}", data);
    println!("reference 1: {reference_1}");
    println!("reference 2: {reference_2}");

    // NOTE: here, * (asterisk) is required to get the Rc pointer's data.
    assert_eq!(*data, 42);
    assert_eq!(*reference_1, 42);
    assert_eq!(*reference_2, 42);
}
