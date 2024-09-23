//! Implement a `Add` trait for a `Millemeters` struct.
//! The `Add` trait should convert `Meters` to `Millemeters` and add them together.

use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Millimeters(u32);
struct Meters(u32);

/// manual implementation of `Add` trait
/// <> bracket is used to specify the 'Rhs' type
/// as required in the `Add` trait definition
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

pub(crate) fn main() {
    assert_eq!(Millimeters(1000) + Meters(1), Millimeters(2000));
}
