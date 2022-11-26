/*
    - Just add the path of the file & import the file as module here
    - Also, the Rust-Analyzer gets activated & gives red flags at errors w/o compiling.
*/

#[path = "../tuts/traits/traits_7.rs"]
mod traits_7;

fn main() {
    traits_7::run();
}
