# Casting

## Overview

Casting is the process of converting a value from one type to another.

In Rust, casting is done using the `as` keyword for primitive types and the `as` keyword with the `From` trait for non-primitive types.

Here are some examples of the `as` keyword usage from the Rust standard library documentation:

```rust
let thing1: u8 = 89.0 as u8;
assert_eq!('B' as u32, 66);
assert_eq!(thing1 as char, 'Y');
let thing2: f32 = thing1 as f32 + 10.5;
assert_eq!(true as u8 + thing2 as u8, 100);
```

These examples show the `as` keyword being used to cast between different types. The `as` keyword in Rust is most commonly used to turn primitive types into other primitive types, but it has other uses that include turning pointers into addresses, addresses into pointers, and pointers into other pointers. It can be seen as the primitive for `From` and `Into`: `as` only works with primitives (`u8`, `bool`, `str`, pointers, …) whereas `From` and `Into` also works with types like `String` or `Vec`.

The `as` keyword can also be used to rename imports in `use` and `extern crate` statements:

```rust
use std::{mem as memory, net as network};
// Now you can use the names `memory` and `network` to refer to `std::mem` and `std::net
```

In this case, `as` is used to rename the `mem` and `net` modules from the `std` library to `memory` and `network`, respectively, within the current scope.
