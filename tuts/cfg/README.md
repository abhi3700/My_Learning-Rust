# Cfg

## Concepts

In Rust, `#[cfg] `is a conditional compilation attribute that allows you to conditionally compile code based on certain conditions. The cfg stands for "configuration" and it is used to specify a configuration option that the Rust compiler can use to determine whether or not to include a particular block of code in the final binary.

For example, you can use `#[cfg(target_os = "linux")]` to specify that a particular block of code should only be compiled if the target operating system is Linux. Similarly, you can use `#[cfg(test)]` to specify that a particular block of code should only be compiled when running tests.

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

`#[cfg(feature="ocw")]` is a conditional compilation attribute that allows you to conditionally compile code based on whether a feature named "ocw" is enabled or not.

Features are a way to conditionally include or exclude code based on the configuration of the Rust crate. They are typically defined in the Cargo.toml file of the crate, and can be enabled or disabled using the `--features` flag when building the crate.

Here's an example of how `#[cfg(feature="ocw")]` can be used in Rust:

```rust
#[cfg(feature="ocw")]
fn main() {
    println!("OCW feature is enabled");
}

#[cfg(not(feature="ocw"))]
fn main() {
    println!("OCW feature is disabled");
}
```

In this example, the `#[cfg(feature="ocw")]` attribute is used to specify that the first main() function should only be compiled if the "ocw" feature is enabled, while the `#[cfg(not(feature="ocw"))]` attribute is used to specify that the second `main()` function should be compiled if the "ocw" feature is disabled.

---

The 🔝 same logic is true for enabling `std` feature unless told with conditional compilation attribute using `#![cfg_attr(not(feature = "std"), no_std)]`.
