//! Filter out 0 & 1 indices of a string
//! Example:
//!
//! "- Impact of the global chip shortage on the development of in-memory chips"

pub(crate) fn main() {
    let s = "- Impact of the global chip shortage on the development of in-memory chips";
    let mut s = s.to_string();
    s.drain(0..2);
    println!("{}", s);
}
