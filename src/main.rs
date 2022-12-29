/*
    - Just add the path of the file & import the file as module here
    - Also, the Rust-Analyzer gets activated & gives red flags at errors w/o compiling.
*/

#[path = "../tuts/traits/traits_10.rs"]
mod traits_10;

fn main() {
    traits_10::main();
}
