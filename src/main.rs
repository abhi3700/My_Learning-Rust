//! Rust By Practice
//! ================
//! https://practice.rs/compound-types/enum.html-1
/*
    - Just add the path of the file & import the file as module here
    - Also, the Rust-Analyzer gets activated (as `Cargo.toml` is added at project root)
    & gives red flags at errors w/o compiling using `cargo check`
*/

#[path = "../tuts/pointer/mutex_1.rs"]
mod mutex_1;

fn main() {
    mutex_1::main();
}
