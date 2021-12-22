## Variables in Rust
### `bool`
no `TRUE`, `FALSE`, `0`, `1` in 

### `char`
no "x", only single quotes

### `signed int`
`i8`, `i16`, `i32`, `i64`, `i128` are fixed size signed integer types

> By default, the integer type in Rust is `i32`.

```rs
let x = 10;		// here by default, x is of type `i32`
```

### `unsigned int`
`u8`, `u16`, `u32`, `u64`, `u128` are fixed size unsigned integer types

### `isize`, `usize`
Pointer sized signed and unsigned integer types. The actual size depends on platforms 32-bit on 32-bit architecture, 64-bit for 64-bit architecture.

### `float`
f32, f64

> By default, the float type in Rust is `f64`.

```rs
let x = 10.5;		// here by default, x is of type `f64`
```

> Should avoid using f32, unless you need to reduce memory consumption badly or if you are doing low-level optimization, when targeted hardware does not support for double-precision or when single-precision is faster than double-precision on it.

### string
[String vs str](https://stackoverflow.com/a/44407956/6774636)

* A Rust `String` is like a `std::string`; it owns the memory and does the dirty job of managing memory.
* A Rust `&str` is like a `char*` (but a little more sophisticated); it points us to the beginning of a chunk in the same way you can get a pointer to the contents of `std::string`.

```rs
// The following code needs to copy the literal string into the String managed memory:
let a: String = "hello rust".into();
```
