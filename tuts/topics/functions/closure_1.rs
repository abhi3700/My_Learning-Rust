//! # Closure
//!
//! > Closure in Rust is like Arrow in JS
//!
//! ----------
//! Closure:
//! ```rs
//! let add_num = || -> {
//!
//! };
//! ```
//!
//! ----------
//! Arrow:
//! ```js
//! let add_num = () => {
//!
//! };
//! ```

pub fn main() {
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
