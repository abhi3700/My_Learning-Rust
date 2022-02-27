# Calculator

## About
* It's a calculator App based on CLI

## Build
* Check error w/o build: `$ cargo check`
* Compile optimized: `$ cargo build --release`

## Output
* Add `1` & `2.5`
```console
❯ cargo run -- 1 + 2.5
   Compiling calculator v0.1.0 (/Users/abhi3700/F/coding/github_repos/My_Learning-Rust/pro/calculator)
warning: unused import: `Args`
 --> src/main.rs:6:22
  |
6 | use std::env::{args, Args};         // `Args` is optional to use
  |                      ^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: `calculator` (bin "calculator") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.37s
     Running `/Users/abhi3700/F/coding/github_repos/My_Learning-Rust/pro/calculator/target/debug/calculator 1 + 2.5`
Result: 3.5
```