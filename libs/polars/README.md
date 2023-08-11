# Polars

Fast multi-threaded DataFrame library in Rust | Python | Node.js

## Overview

Polars is a DataFrame library that is fast and memory efficient. It is written in Rust and has bindings for Python and Node.js. It is built on top of the Apache Arrow columnar memory format and uses SIMD instructions to speed up computations. It is designed to be used in production and is used in production at [RustPython](https://github.com/RustPython/RustPython)

## Getting Started

- Add the following to your `Cargo.toml` file:

  ```toml
  [dependencies]
  polars = "0.31.1"
  ```

  Also, can add more features with `features` param here.

- For code, refer [this](./demo/src/main.rs) file.

## References

- polars lib:
  - [crates.io](https://crates.io/crates/polars)
  - [Github](https://github.com/pola-rs/polars)
  - [API documentation | docs.rs](https://pola-rs.github.io/polars/polars/)
  - [API reference | much more detailed](https://pola-rs.github.io/polars/py-polars/html/reference/index.html#)
  - [Documentation](https://www.pola.rs/)
  - [Book](https://pola-rs.github.io/polars-book/)
