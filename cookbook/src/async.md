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
cargo add tokio --features full
```

It then is going to add macro features when annotating the function (especially `main` function) with `#[tokio::main]`.

To understand **concurrency**, let's go through [this](https://github.com/abhi3700/My_Learning-Rust/blob/23ed14f3795741561c100251fc3e09cf40e6227d/tuts/examples/topics/async/README.md#L454).

> For sleep inside an async task, don't use `std::thread::sleep(Duration::from_secs(..))`. It blocks the task. Instead use `tokio::time::sleep(..)`.

For details, follow this [guide](https://github.com/abhi3700/My_Learning-Rust/blob/main/tuts/examples/topics/async/README.md).
