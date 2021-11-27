# Cargo

## About
* Every cargo project has a config file: `Cargo.toml`
* There is a main file created inside `./src/` - `main.rs`
* After build, the cargo generates a `Cargo.lock` file & a `target/` folder. Just add this into `.gitignore` file for a `.git` repo.
* The output file is present inside `./target/build/debug/`

## Commands
* Create a new project using `$ cargo new <project-name>`
* Build the project using `$ cargo build` in the root of the project (containing `Cargo.toml`).
* Run the project using `$ cargo run` in the root of the project (containing `Cargo.toml`).

> NOTE: In the run process, by default first `build` is executed & then the output is run, if everything is Ok. If the file has changed, then in the console, it will show as `compiling...`, otherwise, simply output the result.

> Run `$ cargo build` command is optional.

* To make sure the file compiles, check your code using `$ cargo check`

> It happens pretty quickly than the `build` time. E.g. `build`: 0.91s, `check`: `0.29s`.

## References
* [Rust: Hello Cargo](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)