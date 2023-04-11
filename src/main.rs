/*
    - Just add the path of the file & import the file as module here
    - Also, the Rust-Analyzer gets activated (as `Cargo.toml` is added at project root)
    & gives red flags at errors w/o compiling using `cargo check`
*/

#[path = "../tuts/pointer/box_3.rs"]
mod box_3;

fn main() {
    box_3::main();
}
