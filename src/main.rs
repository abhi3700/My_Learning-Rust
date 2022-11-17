/*
    - Just add the path of the file & import the file as module here
    - Also, the Rust-Analyzer gets activated & gives red flags at errors w/o compiling.
*/

#[path = "../tuts/collection/vec_of_tuple.rs"]
mod vec_of_tuple;

fn main() {
    vec_of_tuple::run();
}
