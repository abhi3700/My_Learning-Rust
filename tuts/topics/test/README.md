# How to Write Test in Rust

## Coding

### Function

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

---

In a test module attributed with `#[cfg(test)]`, one can use `super::*` to import the parent module's functions.

```rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mytest() {
        // …
    }
}
```

### CLI

> Below, `test_function_name` wherever used follows regex pattern matching.

CLI commands to run test(s):

- Run all tests in the project.

```sh
cargo test
```

- Run a specific test function.

```sh
cargo test <test_function_name>
```

> NOTE: `--exact` flag can be used to run the test exactly matching the name.

- Run all tests in a specific module inside a package in rust workspace.

```sh
cargo test -p <package_name> <module_name>
```

- Run a specific test function in a specific module inside a package in rust workspace.

```sh
cargo test -p <my_package> <module_name>::<test_function_name>
```

- Run all the ignored tests.

```sh
cargo test -- --ignored
```
