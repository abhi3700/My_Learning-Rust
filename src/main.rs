/*
    - Just add the path of the file & import the file as module here
    - Also, the Rust-Analyzer gets activated & gives red flags at errors w/o compiling.
*/

#[path = "../tuts/traits/traits_1.rs"]
mod traits_1;

fn main() {
    traits_1::main();
}
