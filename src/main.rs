/*
    - Just add the path of the file & import the file as module here
    - Also, the Rust-Analyzer gets activated (as `Cargo.toml` is added at project root)
    & gives red flags at errors w/o compiling using `cargo check`
*/

#[path = "../tuts/traits/traits_12b_bounds.rs"]
mod traits_12b_bounds;

fn main() {
    traits_12b_bounds::main();
}
