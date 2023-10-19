---
title: Async Rust
---

There are mainly 2 packages for async Rust:

- [tokio](https://crates.io/crates/tokio/)
- [async-std](https://crates.io/crates/async-std)

[All Feature flags](https://docs.rs/mongodb/latest/mongodb/index.html#all-feature-flags)

## Tokio

In tokio crate, always remember to add the crate with full features at least like this:

```sh
$ cargo add tokio --features full
```

It then is going to add macro features when annotating the function (especially `main` function) with `#[tokio::main]`.
