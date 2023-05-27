# Rust Generics

## Overview

Generics are a way to abstract over types. They allow us to write code that can work with any type.

## Generic Functions

## Generic Structs

## Generic Enums

## Generic Implementations

> `impl<T>` is suffixed with `<T>` to indicate that we are implementing methods on a generic type. So, wherever there is a generic struct, `impl` is followed by `<T>` and the methods are defined within the `impl` block.

### "implement methods for generic struct"

```rust
struct MyStruct<T> {
    // fields...
}

impl<T> MyStruct<T> {
    // method implementations...
}
```

### "implement a trait for generic struct"

```rust
struct MyStruct<T> {
    // fields...
}

trait MyTrait {
    // method signatures...
}

impl<T> MyTrait for MyStruct<T> {
    // method implementations...
}
```

### "implement a generic trait for a struct"

```rust
struct MyStruct {
    // fields...
}

trait MyTrait<T> {
    // method signatures...
}

impl MyTrait<T> for MyStruct {
    // method implementations...
}
```

## Generic Traits

- [generic_trait](./g_trait_struct.rs)
