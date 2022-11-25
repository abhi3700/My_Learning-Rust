/*
Rust automagically looks for it inside the file, if doesn't find it,
looks for a file with the module name in the same folder (in this case src/)
and if still doesn't find it looks for a folder with the module name and a file mod.rs inside,
there it looks for the code.
*/

mod something;

use crate::something::a::*;
use crate::something::b::*;

fn main() {
    let first = A { a: 67 };

    println!("{}", first.a);

    let second = B { b: 69 };

    println!("{}", second.b);
}
