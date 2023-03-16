/*
    - Just add the path of the file & import the file as module here
    - Also, the Rust-Analyzer gets activated (as `Cargo.toml` is added at project root)
    & gives red flags at errors w/o compiling using `cargo check`
*/

#[path = "../tuts/macros/macros_1.rs"]
mod macros_1;

fn main() {
    macros_1::main();
}
