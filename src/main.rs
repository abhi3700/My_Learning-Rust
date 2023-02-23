/*
    - Just add the path of the file & import the file as module here
    - Also, the Rust-Analyzer gets activated (as `Cargo.toml` is added at project root)
    & gives red flags at errors w/o compiling using `cargo check`
*/

// #[path = "../tuts/ownership/dereference_6.rs"]
// mod dereference_6;

fn main() {
    // dereference_6::main();

    let x = 5;

    let add_num = |k: i32| -> i32 {
        let mut x = x; // added/cloned a/into a local variable to avoid mutation of global variable
        x += k;
        x
    };

    println!("{:?}", add_num(10));
    println!("{}", x);
}
