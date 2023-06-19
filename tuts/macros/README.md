# Macros

## Overview

Macros are a powerful feature of Rust. They allow you to write code that writes code. This is a very powerful feature, but it can also be a bit confusing. This tutorial will walk you through the basics of writing macros.

It is really helpful to create libraries for blockchain projects like Substrate pallet, etc.

In Rust, macros are executed at compile time and they expand into new pieces of code that the compiler will then need to further process [source](https://stackoverflow.com/questions/73186696/is-there-any-performance-difference-between-macros-and-functions-in-rust). In terms of performance, macros are similar to `#[inline(always)]` functions. This can be good or bad for performance, depending on various characteristics like the number of calls to the code, the size of the code, or instruction cache pressure. Therefore, it's recommended to benchmark before making a decision.

However, macros do not inherently reduce code duplication at runtime. They can reduce code duplication in the source code, as they allow you to write reusable chunks of code, but the expanded macro code is still present in the compiled program for each invocation of the macro. This could potentially lead to larger binary sizes, especially if the macro expands to a large amount of code and is used many times. It's also worth noting that compile-time performance of macros is usually slower as they are compiled for each invocation【12†source】.

So while macros can help reduce code duplication in the source code and can sometimes be beneficial for runtime performance, they do not reduce code duplication at runtime in the way that functions do. If you can use a function instead of a macro, that is usually preferable, as it can also be marked `#[inline(always)]` if deemed good for performance, while using more familiar syntax and faster compile times【12†source】.

- Apply `#[derive(Debug)]` for making the struct, enum printable
- Apply `#[derive(Clone)]` for making the struct, enum copyable.
- Use this globally in `src/main.rs` to ignore unused code, variables

```rust
#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    // ...
}
```

- A function is marked with #[allow(dead_code)], which suppresses the Rust compiler warning for unused code. This is typically used when the function is expected to be used in the future or when it is part of a public API that isn't currently being used in the local codebase.

  - Suppose, there is a util function and I am not using it anywhere. So, I can mark it as `#[allow(dead_code)]` to avoid the compiler's warning.

  ```rust
  #[allow(dead_code)]
  fn handle_task() {}
  ```

  Further explanation:

  `#[allow(dead_code)]` 🚦

  This line is like a "do not disturb" sign 🚫. It tells the Rust compiler not to worry if this function isn't being used right now.

- Same goes for unused variables:

  ```rust
  #[allow(unused_variables)]
  let v = vec![1, 2, 2, 3, 4, 4, 5, 6, 7, 7];
  ```

## References

- [Learn Macros In Rust](https://github.com/tfpk/macrokata)
- [Learn Proc macros](https://github.com/dtolnay/proc-macro-workshop#rust-latam-procedural-macros-workshop)
