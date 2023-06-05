fn main() {
    // in markdown format
    let input_markdown = r#"
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

This trait is implemented for various types in the Rust standard library to provide overloading of the `+` operator. When you define `a + b` in your Rust code, under the hood it is calling the `add` method of the `Add` trait for the type of `a` (and `b` if it's different), like `a.add(b)`."#.to_string();

    // in plain text
    let output_plaintxt = markdown_to_text::convert(&input_markdown);

    println!("{}", output_plaintxt);
}
