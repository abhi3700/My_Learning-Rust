# Rust Generics

## Overview

Generics are a way to abstract over types. They allow us to write code that can work with any type.

Generics are a way to reduce the need to write repetitive code and instead delegate this task to the compiler while also making the code more flexible. Many languages support some way to do this, even though they might call it something different.

Using generics, we can write code that can be used with multiple data types without having to rewrite the same code for each data type, making life easier and coding less error-prone.

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

/// when `MyTrait` is implemented for `MyStruct<T>`
impl<T> MyTrait for MyStruct<T> {
    // method implementations...
}

/// when `MyTrait` is implemented for `MyStruct<T>` where `T` implements `SomeOtherTrait`
impl<T> MyTrait for MyStruct<T> where T: SomeOtherTrait {
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
