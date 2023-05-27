# Rust Generics

## Overview

Generics are a way to abstract over types. They allow us to write code that can work with any type.

## Generic Functions

## Generic Structs

## Generic Enums

## Generic Implementations

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

- [generic_trait](./generic_trait.rs)
