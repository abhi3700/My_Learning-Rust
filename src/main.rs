//! Rust By Practice
//! ================
//! https://practice.rs/compound-types/enum.html-1
/*
    - Just add the path of the file & import the file as module here
    - Also, the Rust-Analyzer gets activated (as `Cargo.toml` is added at project root)
    & gives red flags at errors w/o compiling using `cargo check`
*/

// #[path = "../tuts/generics/generics_11.rs"]
// mod generics_11;

use std::time::{SystemTime, UNIX_EPOCH};
fn main() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("{}", now);
}
