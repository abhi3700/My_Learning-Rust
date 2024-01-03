# Error Handling in Rust

## Coding

- Define custom errors like this:

  ```rs
  enum ParseIntError;

  #[derive(Debug)]
  enum DoubleError {
      EmptyVec,
      // We will defer to the parse error implementation for their error.
      // Supplying extra info requires adding more data to the type.
      Parse(ParseIntError),
  }
  ```

  And then define the main function like this:

  ```rust
  fn main() -> Result<(), DoubleError> {
    match x {
      Ok(_) => Ok(()),
      Err(e) if x == 3 => Err(DoubleError::Parse(e)),
      Err(e) if x != 3 => Err(DoubleError::EmptyVec),
    }
  }
  ```

- If we want to attach a message to the error, we can use a crate `thiserror` as shown [here](./this_error.rs).
- **`Option` vs `Result`**

  | **Option**                                                                                                                                                                                  | **Result**                                                                                                                             |
  | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
  | Some or None                                                                                                                                                                            | Ok or Err                                                                                                                          |
  | An optional value can have either Some value or no value/ None.                                                                                                                         | A result can represent either success/ Ok or failure/ Err                                                                          |
  | The Option type is a way to use Rust’s type system to express the possibility of absence                                                                                                | Result expresses the possibility of error                                                                                          |
  | mainly used for var, function output. For struct, the parameters can have Option type. E.g. In full name, middle_name can be missing for cases, so define `middle_name: Option<String>` | mainly used for operation, function. As normally a variable won't have Err unless there is some calculation involved with this var |
  | Don't want to print the exact issue as `None` doesn't have anything as param unlike `Some(T)`                                                                                           | Want to print the exact issue as `Err(E)` contains the message inside                                                              |
  | E.g. [`./tuts/error_handling/opt_1.rs`](./opt_1.rs)                                                                                                                                                        | E.g. [`./tuts/error_handling/res_1.rs`](./res_1.rs)                                                                                                   |

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

---

**When to use `unwrap`**:

[Source](https://owengage.com/writing/2021-08-30-how-to-think-of-unwrap/)

- When you know better than the compiler.
- When you don't care if some code panics like in tests.
- When you have no expectation of recovering from the error.
- In rust lib codebase, unwrap shouldn’t be used, rather `match` & `?` is permissible. And then, one should write code for max. test coverage.
- Use `expect` instead of `unwrap` to give as much context as possible in rust bin codebase.
  > Then, why should one use `unwrap` at all? Because, it is concise and doesn't need any string message.

---

**How to swallow/consume errors?**

Using `map_err`:

Let's say there is a function `fn foo() -> Result<T, E>`. Now, if we want to call this function and ignore the error, we can do like this:

```rust
pub fn transfer(
  &mut self,
  caller: String,
  to: String,
  amount: u128,
 ) -> Result<(), &'static str> {
   // some code
  let new_caller_balance = caller_balance.checked_sub(amount).ok_or("Insufficient funds.")?;
  // some code
}

fn main() {
  let mut balances = Balances::new();
  assert_eq!(runtime.balances.balance(&"alice".to_string()), 100);
  
  // transfer 101 from alice to bob -> should fail as alice has only 100 tokens ❌
  let _res = balances.transfer("alice".to_string(), "bob".to_string(), 101).map_err(|e| eprintln!("Error: {}", e))?;
  // NOTE: The next step didn't stop as the previous step didn't panic.
  // transfer 20 from alice to bob ✅
  let _res = balances.transfer("alice".to_string(), "bob".to_string(), 20).map_err(|e| eprintln!("Error: {}", e))?;
  assert_eq!(runtime.balances.balance(&"alice".to_string()), 80); // alice has 80 balance
  assert_eq!(runtime.balances.balance(&"bob".to_string()), 20);   // bob has 20 balance
}
```

When this code is run on terminal:
  
```bash
$ cargo r
Error: Insufficient funds.
```

Basically, there is no panic. The error is swallowed.

## References

- [Rust: Error Handling](https://www.youtube.com/watch?v=y3wUCb-uS3g)
