# Error

## About

## Coding

- Define custom errors like this:

```rs
#[derive(Debug)]
enum DoubleError {
    EmptyVec,
    // We will defer to the parse error implementation for their error.
    // Supplying extra info requires adding more data to the type.
    Parse(ParseIntError),
}
```

- `Option` vs `Result`

| Option                                                                                                                                                                                  | Result                                                                                                                             |
| --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| Some or None                                                                                                                                                                            | Ok or Err                                                                                                                          |
| An optional value can have either Some value or no value/ None.                                                                                                                         | A result can represent either success/ Ok or failure/ Err                                                                          |
| The Option type is a way to use Rust’s type system to express the possibility of absence                                                                                                | Result expresses the possibility of error                                                                                          |
| mainly used for var, function output. For struct, the parameters can have Option type. E.g. In full name, middle_name can be missing for cases, so define `middle_name: Option<String>` | mainly used for operation, function. As normally a variable won't have Err unless there is some calculation involved with this var |
| Don't want to print the exact issue as `None` doesn't have anything as param unlike `Some(T)`                                                                                           | Want to print the exact issue as `Err(E)` contains the message inside                                                              |
| E.g. "./tuts/error_handling/opt"                                                                                                                                                        | E.g. "./tuts/error_handling/res"                                                                                                   |

---

**`expect` vs `ok_or`**:

- `expect` gives T if `Some(T)` else panics with the given message. `ok_or` gives `Ok(T)` if `Some(T)` else `Err(E)`.

<u>Example</u>:

```rs
let x = Some("value").expect("the world is ending"); // x: &str = "value"
let y = None.expect("the world is ending"); // this will panic!

let x = Some("value").ok_or("the world is ending"); // x: Result<&str, &str> = Ok("value")
let y = None.ok_or("the world is ending"); // y: Result<&str, &str> = Err("the world is ending")
```

<u>Example-2</u>: From testing context (taken from substrate tryouts):

```rs
/// pallet balances code inside transfer
let new_caller_balance = caller_balance.checked_sub(amount).ok_or("Not enough funds.")?;

/// test code
assert_eq!(
    balances.transfer("alice".to_string(), "bob".to_string(), 101),
    Err("insufficient balance")
);
// OR
assert!(balances.transfer("alice".to_string(), "bob".to_string(), 101).is_err());
```

> Here, this code would have panicked if `ok_or` was not used. I tried with `expect` and the test panicked.

## References

- [Rust: Error Handling](https://www.youtube.com/watch?v=y3wUCb-uS3g)
