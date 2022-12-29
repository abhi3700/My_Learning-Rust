/*
    - Just add the path of the file & import the file as module here
    - Also, the Rust-Analyzer gets activated & gives red flags at errors w/o compiling.
*/

#[path = "../tuts/traits/traits_8.rs"]
mod traits_8;

fn main() {
    traits_8::main();
}
