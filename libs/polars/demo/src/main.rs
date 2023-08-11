use polars::prelude::*;

fn main() {
    let s = Series::new("a", [1, 2, 3, 4, 5]);
    println!("{}", s);
}
