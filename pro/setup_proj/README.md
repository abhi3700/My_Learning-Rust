# Setup Project

## About

This project shows the journey from a single file (`main.rs`) to a multi-file project.

## Coding

Different versions/steps:

- [v0.1](https://github.com/abhi3700/My_Learning-Rust/blob/b61d67426399c231b9d995fab078f91eff470b69/pro/setup_proj/src/main.rs)
- [v0.2 added module in same file & imported](https://github.com/abhi3700/My_Learning-Rust/blob/0bd6f7cb731868fa6d773a6537a3183a07accfea/pro/setup_proj/src/main.rs)
- [v0.2.1 added pub to an struct inside another module in same file](https://github.com/abhi3700/My_Learning-Rust/blob/b75b3d0465d3772bf1c9cf9f93259d8b3671c922/pro/setup_proj/src/main.rs)
- [v0.3 shifted module to another file](https://github.com/abhi3700/My_Learning-Rust/commit/31c2bc23e2605e432f831acb9aaf2a86de4381c4)
- [v0.4 renamed something.rs as mod.rs & put inside something/](https://github.com/abhi3700/My_Learning-Rust/commit/0b699966039a5a87b71338cef8d1d6d5285772af)
- [v0.4.1 bifurcated mod into a, b structs](https://github.com/abhi3700/My_Learning-Rust/commit/083d7a14a338e86d8a1fae17cf7746191ba0a547)
- [v0.5 Rust library with an executable file](https://github.com/abhi3700/My_Learning-Rust/commit/7f3efe239ddc418bc75ba71797fb6677d99d4527)
- [v0.6 rust workspace](https://github.com/abhi3700/My_Learning-Rust/commit/a18110710902f6cfe93393edc1587a7cfa3a90cc)
  - 2 workspaces: db_stuff (lib) & ftbmh (pkg) created
  - ftbmh depends on 'db_stuff' lib
  - cargo new --lib db_stuff
  - cargo new ftbmh

## Reference

- [Rust: Project structure example step by step](https://dev.to/ghost/rust-project-structure-example-step-by-step-3ee)
