# Serialize & Deserialize

This crate provides a set of traits and derive macros to serialize and deserialize Rust data structures to and from JSON.

## Usage

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

## Reference

- [Parsing JSON in Rust using serde and reqwest](https://www.youtube.com/watch?v=ogpE4hviXyA&t=37s)
