/*
    - Just add the path of the file & import the file as module here
    - Also, the Rust-Analyzer gets activated & gives red flags at errors w/o compiling.
*/

#[path = "../tuts/pointer/array_addr.rs"]
mod array_addr;

fn main() {
    array_addr::main();
}
