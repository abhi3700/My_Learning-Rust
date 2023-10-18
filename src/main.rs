//! Rust By Practice
//! ================
//! https://practice.rs/compound-types/enum.html-1
/*
    - Just add the path of the file & import the file as module here
    - Also, the Rust-Analyzer gets activated (as `Cargo.toml` is added at project root)
    & gives red flags at errors w/o compiling using `cargo check`
*/
#[path = "../tuts/features/feature1.rs"]
mod feature1;

fn main() {
    feature1::main();
}
