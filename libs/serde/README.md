# Serialize & Deserialize

This crate provides a set of traits and methods to serialize and deserialize Rust data structures (`struct`) to and from JSON.

## Overview

Whenever you want to send data over the network or save it to a file, you need to serialize it into a format that can be transferred or stored. The most popular format for serializing data is JSON. It's human-readable, easy to parse by computers, and supported by most programming languages.

That's why Rust has a built-in library for serializing and deserializing Rust data structures to and from JSON. It's called `serde` (short for "serializing and deserializing").

---

`serde` is a framework for serializing and deserializing Rust data structures efficiently and generically. It supports two main operations:

- `Serialize` - turning Rust data structures into a JSON string
- `Deserialize` - turning a JSON string into Rust data structures

---

`serde_json` is a Rust library for parsing JSON data. It is an implementation of the `serde` framework. It allows you to parse JSON strings and turn them into Rust data structures. It also allows you to turn Rust data structures into JSON strings. It supports two main operations:

- `to_string` - turning Rust data structures into a JSON string
- `from_str` - turning a JSON string into Rust data structures

## Usage

```sh
$ cargo init
$ cargo add serde --features derive
$ cargo add serde_json
```

### Using `serde`, `serde_json`

1. Add this to your `Cargo.toml`:

   ```toml
   [dependencies]
   serde = { version = "1.0", features = ["derive"] }
   serde_json = "1.0"
   ```

2. Then, add code into `main.rs`.

Examples:

- [lesson-0](./src/lesson_0.rs)

### Using `serde`, `serde_json`, `reqwest`, `tokio`

1. Add this to your `Cargo.toml`:

   Sufficient for JSON response to any Rust type.

   ```toml
   [dependencies]
   serde = { version = "1.0", features = ["derive"] }
   reqwest = {version = "0.11", features = ["json"]}
   tokio = {version = "1.0", features = ["full"]}
   ```

   Sufficient for any type to JSON response.

   ```toml
   [dependencies]
   serde = { version = "1.0", features = ["derive"] }
   serde_json = "1.0"
   reqwest = {version = "0.11", features = ["json"]}
   tokio = {version = "1.0", features = ["full"]}
   ```

2. Then, add code into `main.rs`.

## Coding

There is a scope of renaming the fields of the struct while serializing and deserializing. This can be done by using the `#[serde(rename = "new_name")]` attribute.

```rust
#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    #[serde(rename = "userId")]
    user_id: i32,
    id: Option<i32>,
    title: String,
    completed: bool,
}
```

> This is useful when the field names in the Rust struct and the JSON string are different.

## Reference

- [Parsing JSON in Rust using serde and reqwest](https://www.youtube.com/watch?v=ogpE4hviXyA&t=37s)
