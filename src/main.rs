/*
    - Just add the path of the file & import the file as module here
    - Also, the Rust-Analyzer gets activated (as `Cargo.toml` is added at project root)
    & gives red flags at errors w/o compiling using `cargo check`
*/

#[path = "../tuts/move_copy_clone/clone_2.rs"]
mod clone_2;

fn main() {
    clone_2::main();
}
