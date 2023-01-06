/*
    - Just add the path of the file & import the file as module here
    - Also, the Rust-Analyzer gets activated (as `Cargo.toml` is added at project root)
    & gives red flags at errors w/o compiling using `cargo check`
*/

#[path = "../tuts/lifetimes/lifetime_4b.rs"]
mod lifetime_4b;

fn main() {
    lifetime_4b::main();
}
