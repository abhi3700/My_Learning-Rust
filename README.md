# My_Learning-Rust
Rust programming language

## Installation
### Linux or MacOS
```console
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

## Getting Started
### Code
```rs
fn main() {
	println!("Hello World!");
}
```

### Compile
```console
$ rustc hello.rs
```

### Output
```console
$ ./hello
```

## Concepts
### Basics
#### Primitive types and Variables
1. Various sizes of integers, signed and unsigned (i32, u8, etc.)
1. Floating point types f32 and f64.
1. Booleans (bool)
1. Characters (char). Note these can represent unicode scalar values (i.e. beyond ASCII)

## References
* [Rust by example](https://doc.rust-lang.org/stable/rust-by-example/)
* [Book: The Rust Programming Language](https://doc.rust-lang.org/book/)
* [Rust for Haskell Programmers!](https://mmhaskell.com/rust)
	- [Part 1: Basic Syntax](https://www.mmhaskell.com/rust/syntax)
	- [Part 2: Managing Memory](https://www.mmhaskell.com/rust/memory)
	- [Part 3: Data Types](https://www.mmhaskell.com/rust/data)
	- [Part 4: Cargo Package Manager](https://www.mmhaskell.com/rust/cargo)
	- [Part 5: Collections and Lifetimes](https://www.mmhaskell.com/rust/lifetimes)
