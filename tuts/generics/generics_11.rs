//! Implement a trait to a generic type

use std::{fmt::Display, path::Display};

struct Pair<T> {
    x: T,
    y: T,
}

impl<T: Display> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Pair { x: x, y: y }
    }

    // FIXME: solve this error
    fn square_xy(&self) -> u32 {
        (self.x as u32).pow(2) + (self.y as u32).pow(2)
    }
}

impl Display for Pair<T> {}

pub fn main() {}
