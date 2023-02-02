/*
    - Just add the path of the file & import the file as module here
    - Also, the Rust-Analyzer gets activated (as `Cargo.toml` is added at project root)
    & gives red flags at errors w/o compiling using `cargo check`
*/

#[path = "../tuts/variables/variables_9.rs"]
mod variables_9;

fn main() {
    variables_9::main();
}
