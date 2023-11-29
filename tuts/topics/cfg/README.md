# Cfg

## Overview

In Rust, `#[cfg]` and `#![cfg_attr]` are both conditional compilation attributes, but they are used in slightly different ways.

`#[cfg]` is used to conditionally compile code based on certain conditions. It is typically used to include or exclude code based on the target platform, operating system, features or other build configuration options.

`#![cfg_attr]`, on the other hand, is used to set an attribute on the crate based on certain conditions. It is similar to `#[cfg]`, but instead of conditionally compiling code, it conditionally sets an attribute on the crate.

For example, `#[cfg(not(feature="std"))]` can be used to conditionally include code when the std feature is not enabled, while `#![cfg_attr(not(feature="std"), no_std)]` can be used to set the no_std attribute on the crate when the std feature is not enabled.

In summary, `#[cfg]` is used to conditionally compile code, while `#![cfg_attr]` is used to conditionally set attributes on the crate.

## Concepts

In Rust, `#[cfg]` is a conditional compilation attribute that allows you to conditionally compile code based on certain conditions. The cfg stands for "configuration" and it is used to specify a configuration option that the Rust compiler can use to determine whether or not to include a particular block of code in the final binary.

For example, you can use `#[cfg(target_os = "linux")]` to specify that a particular block of code should only be compiled if the target operating system is Linux based. Similarly, you can use `#[cfg(test)]` to specify that a particular block of code should only be compiled when running tests.

Here's an example of how `#[cfg]` can be used in Rust:

```rust
#[cfg(target_os = "linux")]
fn main() {
    println!("This code will only be compiled on Linux");
}

#[cfg(not(target_os = "linux"))]
fn main() {
    println!("This code will be compiled on any platform except Linux");
}
```

In this example, the `#[cfg(target_os = "linux")]` attribute is used to specify that the first `main()` function should only be compiled on Linux, while the `#[cfg(not(target_os = "linux"))]` attribute is used to specify that the second `main()` function should be compiled on any platform except Linux.

---

`#[cfg(feature = "ocw")]` is a conditional compilation attribute that allows you to conditionally compile code based on whether a feature named "ocw" is enabled or not.

Features are a way to conditionally include or exclude code based on the configuration of the Rust crate. They are typically defined in the Cargo.toml file of the crate, and can be enabled or disabled using the `--features` flag when building the crate.

Here's an example of how `#[cfg(feature = "ocw")]` can be used in Rust:

```rust
#[cfg(feature = "ocw")]
fn main() {
    println!("OCW feature is enabled");
}

#[cfg(not(feature = "ocw"))]
fn main() {
    println!("OCW feature is disabled");
}
```

In this example, the `#[cfg(feature = "ocw")]` attribute is used to specify that the first main() function should only be compiled if the "ocw" feature is enabled, while the `#[cfg(not(feature = "ocw"))]` attribute is used to specify that the second `main()` function should be compiled if the "ocw" feature is disabled.

---

The 🔝 same logic is true for enabling `std` feature unless told with conditional compilation attribute using `#![cfg_attr(not(feature = "std"), no_std)]`.
