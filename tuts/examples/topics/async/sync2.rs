//! This program demonstrates the usage of `Rc` (Reference Counted) smart pointer.
//! `Rc` allows multiple ownership of the same data by keeping track of the number of references to
//! a value which determines whether or not the value is still in use.
//!
//! The code creates a new `Rc` smart pointer to an integer value. It then gets a mutable reference
//! to the data and modifies it. It creates two new references to the original data using `Rc::clone()`.
//! Finally, it prints the original data and the two references to it.
//!
//! Example:
//!
//! ```
//! use std::rc::Rc;
//!
//! // Create a new `Rc` smart pointer to an integer value.
//! let mut data = Rc::new(42);
//! println!("original data [Start]: {}", data);
//!
//! // Get a mutable reference to the data. This is only possible if there is no reference
//! // to the data.
//! let writeable_data = Rc::get_mut(&mut data).unwrap();
//! *writeable_data = 100;
//!
//! // Print the modified value of the original data.
//! println!("The original data is modified as: {}", data);
//!
//! // Create a new reference to the original data using `Rc::clone()`.
//! let reference_1 = Rc::clone(&data);
//! println!("reference 1 to original data: {}", reference_1);
//!
//! // Create another reference to the original data.
//! let reference_2 = Rc::clone(&data);
//! println!("reference 2 to original data: {}", reference_2);
//!
//! // Print the final value of the original data.
//! println!("original data [End]: {}", data);
//! ```
//! Output:
//!
//! ```sh
//! original data [Start]: 42
//! The original data is modified as: 100
//! reference 1 to original data: 100
//! reference 2 to original data: 100
//! original data [End]: 100
//! ```
use std::rc::Rc;

/// Demonstrates the usage of `Rc` (Reference Counted) smart pointer.
/// `Rc` allows multiple ownership of the same data by keeping track of the number of references to
/// a value which determines whether or not the value is still in use.
pub(crate) fn main() {
    // Create a new `Rc` smart pointer to an integer value.
    let mut data = Rc::new(42);
    println!("original data [Start]: {}", data);

    // Get a mutable reference to the data. This is only possible if there is no reference
    // to the data.
    let writeable_data = Rc::get_mut(&mut data).unwrap();
    *writeable_data = 100;

    // Print the modified value of the original data.
    println!("The original data is modified as: {}", data);

    // Create a new reference to the original data using `Rc::clone()`.
    let reference_1 = Rc::clone(&data);
    println!("reference 1 to original data: {}", reference_1);

    // Create another reference to the original data.
    let reference_2 = Rc::clone(&data);
    println!("reference 2 to original data: {}", reference_2);

    // Print the final value of the original data.
    println!("original data [End]: {}", data);
}
