/*
    - Just add the path of the file & import the file as module here
    - Also, the Rust-Analyzer gets activated & gives red flags at errors w/o compiling.
*/

#[path = "../tuts/traits/clone_2.rs"]
mod clone_2;


fn main() {
    clone_2::run();
}