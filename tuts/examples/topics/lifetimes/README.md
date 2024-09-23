# Lifetimes

## Overview

- Understanding lifetimes involves understanding the concept of scopes, and how Rust uses scopes to manage memory and resources. If a variable is passed by reference, it might be dropped from memory (if its scope ends) while the reference is still in use, causing a "dangling reference". Lifetimes prevent this from happening by ensuring that a reference cannot outlive the data it references.
- Lifetimes are another kind of generic that we’ve already been using. Rather than ensuring that a type has the behavior we want, lifetimes ensure that references are valid as long as we need them to be. [source](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#preventing-dangling-references-with-lifetimes)
- Rust doesn't allow _dangling pointer_ by design. This means that any variable, struct, enum, etc can't live more than the lifetime of the referenced type

```rs
struct Config {

}

// INCORRECT ❌
struct App {
    config: &Config     // `Config` used as reference
}

// CORRECT ✅
/// Here, it is used as lifetime ownership of the code.
struct App<'a> {
    config: &'a Config
}
```

- `lifetimes` are a compile-time feature and don’t exist at runtime.
