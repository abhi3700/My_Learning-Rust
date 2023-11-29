# How to Write Test in Rust

## Coding

Add `#[test]` attribute to a function to make it a test function.

```rs
#[test]
fn mytest() {
    // …
}
```

---

Ignore a test. One can also add a reason to the `ignore` attribute.

```rs
#[test]
#[ignore = "not yet implemented"]
fn mytest() {
    // …
}
```

---

Should panic with a expected message.

```rs
#[test]
#[should_panic(expected = "Timestamp must be updated only once in the block")]
fn mytest() {
    // …
}
```
