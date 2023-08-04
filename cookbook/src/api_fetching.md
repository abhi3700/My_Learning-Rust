---
title: API Fetching
---

In order to fetch API GET request, we use the `reqwest` crate in Rust.

There are 4 versions of the code for a simple API fetching like random number generation.

- [v1](../../libs/reqwest_crate/src/v1.rs): Using `reqwest` crate with `async`/`await` and and `tokio` runtime
- [v2](../../libs/reqwest_crate/src/v2.rs): Decoupling the `async`/`await` code from the main function
- [v3](../../libs/reqwest_crate/src/v3.rs): Add `std::env` module to get the URL from the environment variable set in CLI.
- [v4](../../libs/reqwest_crate/src/v4.rs): Add `dotenv` crate to load the `.env` file and get the URL from the environment variable set in the `.env` file.

We need to add 3 crates in total till `v4`:

```toml
[dependencies]
# cargo add dotenv
dotenv = "0.15.0"
# cargo add reqwest --features json
reqwest = {version ="0.11.18", features = ["json"]}
# cargo add tokio --features full
tokio = {version = "1.29.1", features = ["full"]}
```

> alphabetically sorted in Rust
