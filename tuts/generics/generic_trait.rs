//! # Generic trait
//!
//! ## Observation:
//! The issue you're encountering is because Rust doesn't know which implementation
//! of `MyTrait` to use for `Counter`. You've provided two implementations
//! for `MyTrait`: one for `i32` and another for `u64`. In the `main` function,
//! when you call `c.get()`, Rust doesn't know which implementation to use.
//!
//! To fix this, you can use a fully qualified path to specify the expected type. In this case, you can specify whether you want the `get()` function from `MyTrait<i32>` or `MyTrait<u64>`.
//!
//! ## Solution:
//! In this code, `<Counter as MyTrait<i32>>::get(&c)` is used to explicitly call the `get()` function from the `i32` implementation of `MyTrait` for `Counter`. If you want to use the `u64` implementation instead, you can replace `<Counter as MyTrait<i32>>::get(&c)` with `<Counter as MyTrait<u64>>::get(&c)`.

/// Generic trait
trait MyTrait<T> {
    fn get(&self) -> T;
}

struct Counter {}

/// implement i32 for Counter
impl MyTrait<i32> for Counter {
    fn get(&self) -> i32 {
        10
    }
}

/// implement u64 for Counter
impl MyTrait<u64> for Counter {
    fn get(&self) -> u64 {
        102_324_234
    }
}

/// implement String for Counter
impl MyTrait<String> for Counter {
    fn get(&self) -> String {
        "Hello".to_string()
    }
}

/// main
pub fn main() {
    let c = Counter {};
    // println!("{}", c.get()); // ❌ type annotations needed. Rust doesn't know which implementation
    // of `MyTrait` to use for `Counter`. You've provided two implementations
    // for `MyTrait`: one for `i32` and another for `u64`
    println!("{}", <Counter as MyTrait<i32>>::get(&c)); // --> 10
    println!("{}", <Counter as MyTrait<u64>>::get(&c)); // --> 102324234
    println!("{}", <Counter as MyTrait<String>>::get(&c)); // --> Hello
}
