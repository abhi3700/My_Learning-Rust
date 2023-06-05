use base64::{engine::general_purpose, Engine as _};

fn main() {
    let input_txt = r#"
The `Add` trait is defined within the `std::ops` module in Rust's standard library. As of my knowledge cutoff in September 2021, it is defined as follows:

```rust
pub trait Add<RHS=Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}
```

Let's break down the elements of this trait:

- `pub trait Add<RHS=Self>`: This declares a public trait named `Add`. The `RHS=Self` part is a default type parameter—it means that if no specific type is provided for `RHS` (which stands for "right-hand side"), it will default to the type implementing the trait (`Self`).

- `type Output;`: This is an associated type. It declares a type that is associated with each implementation of the trait. In the case of the `Add` trait, `Output` represents the type that results from the addition operation.

- `fn add(self, rhs: RHS) -> Self::Output;`: This is the method that types implementing the `Add` trait must provide. It takes two parameters: `self` and `rhs` (the right-hand side of the operation). `self` is consumed in this operation because `self` is taken by value (not as a reference). This makes sense for many types because, for example, adding two numbers doesn't change the numbers themselves—you get a new number as a result. The method returns a value of the associated `Output` type.

This trait is implemented for various types in the Rust standard library to provide overloading of the `+` operator. When you define `a + b` in your Rust code, under the hood it is calling the `add` method of the `Add` trait for the type of `a` (and `b` if it's different), like `a.add(b)`."#;

    // println!("{}", input_txt);

    let b64 = general_purpose::STANDARD.encode(input_txt.as_bytes());

    println!("encoded as: {}", b64);
    let b64_decoded = general_purpose::STANDARD.decode(b64.as_bytes()).unwrap();
    println!("\ndecoded bytes as: {:?}", b64_decoded);

    // let _b64_decoded_array: &[u8] = &b64_decoded;

    let s = match std::str::from_utf8(&b64_decoded) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    println!("\ndecoded string as: {}", s);
}
