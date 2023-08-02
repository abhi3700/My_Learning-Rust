//! Rust By Practice
//! ================
//! https://practice.rs/compound-types/enum.html-1
/*
    - Just add the path of the file & import the file as module here
    - Also, the Rust-Analyzer gets activated (as `Cargo.toml` is added at project root)
    & gives red flags at errors w/o compiling using `cargo check`
*/
#[path = "../tuts/traits/traits_interoperability.rs"]
mod traits_interoperability;

// fn main() {
//     traits_interoperability::main();
// }

use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("c", 2);
    map.insert("a", 3);
    map.insert("b", 1);
    map.insert("f", 4);
    map.insert("i", 9);
    map.insert("g", 6);

    let mut vec: Vec<_> = map.into_iter().collect();
    println!("{:?}", vec);
    vec.sort_by_key(|&(_, v)| v);
    println!("{:?}", vec);

    let sorted_map: HashMap<_, _> = vec.into_iter().collect();
    println!("{:?}", sorted_map);
}
