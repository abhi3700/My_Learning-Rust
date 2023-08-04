---
title: Environment Variables
---

In the std module: `std::env`, there is no option of providing the `.env` file path from where the "URL" environment variable has to be parsed. So, we need to use the `dotenv` crate to load the `.env` file.

This is how we do it...

Here is the repo file structure:

```
.
├── .env
├── Cargo.lock
├── Cargo.toml
├── README.md
├── src
│   ├── main.rs
└── target
```

In the `Cargo.toml` file, add the `dotenv` crate as a dependency:

```toml
[dependencies]
# cargo add dotenv
dotenv = "0.15.0"
```

Create a `.env` file in the root directory of the project and add the environment variable:

```sh
# .env
URL="https://www.random.org/integers/?num=1&min=1&max=100&col=1&base=10&format=plain&rnd=new"
```

In the main file, load the `.env` file using the `dotenv` crate and then use the `std::env` module to get the environment variable:

```rust
/// src/main.rs

// load the .env file
// path has to be w.r.t `Cargo.toml` file
dotenv::from_path("./.env").expect("Failed to load .env file");

use std::env;

let url: String = env::var("URL").expect("URL not found in .env file");
```

That's it!
