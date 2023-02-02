/*
    - Just add the path of the file & import the file as module here
    - Also, the Rust-Analyzer gets activated (as `Cargo.toml` is added at project root)
    & gives red flags at errors w/o compiling using `cargo check`
*/

#[path = "../tuts/iterator/collect_4_opt.rs"]
mod collect_4_opt;

fn main() {
    collect_4_opt::main();
}
