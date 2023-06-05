# Base64

Encoding/Decoding Base64 in Rust.

## Usage

Add this to your `Cargo.toml`:

```sh
$ cargo add base64
```

and in `main.rs` file:

the code snippet is [here](./demo/src/main.rs)

## Steps

1. Parse the input string i.e. in raw format using `r#"..."#` syntax. You can have any format of string like markdown or anything.
   > In order to get any markdown, just use any `markdown_to_text` converter. Please refer this example [here](../markdown_to_text/demo/src/main.rs)
2. Convert the input string to bytes using `as_bytes()` method.
3. Encode the bytes using `encode()` method.
4. Decode the bytes using `decode()` method.
5. Convert the bytes to string using `std::str::from_utf8()` method.

## References

- [base64](https://crates.io/crates/base64)
