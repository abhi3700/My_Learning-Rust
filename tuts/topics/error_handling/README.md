# Error Handling in Rust

Mainly 2 types: Recoverable and Unrecoverable using Option and Result along with some macros `panic!`.

## Coding

- Recoverable using `Result`

  - e.g. `Result::Err("burn and crash")` in case of function return type.

The `try-catch` can be implemented like this:

```rs
fn main() {
    // the output is of type `Result<File, Error>`
    let f = File::open("hello.txt");
    match f {
        Ok(success) => println!("{:?}", success),
        Err(failure) => panic!("file is not found: {:?}", failure),
    };
}
```

in analogous to:

```js
try {
  const f = File.open("hello.txt");
  console.log(f);
} catch (e) {
  console.log(e);
}
```

Understand the following examples sequentially:

[recoverable_err_1a.rs](./recoverable_err_1a.rs)

[recoverable_err_1b.rs](./recoverable_err_1b.rs)

[recoverable_err_1c.rs](./recoverable_err_1c.rs)

---

- Unrecoverable using `panic`
  - e.g. `panic!("burn and crash")` in case of array out of bound error.

```rs
fn run() {
  panic!("burn and crash");
}

fn main() {
  run();
}
```

---

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

### `expect` vs `ok_or`

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

### When to use `unwrap`

[Source](https://owengage.com/writing/2021-08-30-how-to-think-of-unwrap/)

- When you know better than the compiler.
- When you don't care if some code panics like in tests.
- When you have no expectation of recovering from the error.
- In rust lib codebase, unwrap shouldn’t be used, rather `match` & `?` is permissible. And then, one should write code for max. test coverage.
- Use `expect` instead of `unwrap` to give as much context as possible in rust bin codebase.
  > Then, why should one use `unwrap` at all? Because, it is concise and doesn't need any string message.

### How to swallow/consume errors?

Using `map_err`:

Let's say there is a function `fn foo() -> Result<T, E>`. Now, if we want to call this function and ignore the error, we can do like this:

**M-1: swallow without `Ok()` arm.**

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

OR

**M-2: swallow with having both `Ok()` arm. Use `match` here.**

```rust
pub fn transfer(
  ...
  ...) {
   // some code

}

fn main() {
  let mut balances = Balances::new();
  assert_eq!(runtime.balances.balance(&"alice".to_string()), 100);
  
  // transfer 101 from alice to bob -> should fail as alice has only 100 tokens ❌
  match runtime.balances.transfer(&alice, &bob, 101) {
    Ok(_) => {
    let _ = runtime.system.inc_nonce(&alice);
    },
    Err(e) => eprintln!("Error: {}", e),
  };
  // NOTE: The next step didn't stop as the previous step didn't panic.
  // transfer 20 from alice to bob ✅
  match runtime.balances.transfer(&alice, &charlie, 20) {
    Ok(_) => {
    let _ = runtime.system.inc_nonce(&alice);
    },
    Err(e) => eprintln!("Error: {}", e),
  };
  assert_eq!(runtime.balances.balance(&"alice".to_string()), 80); // alice has 80 balance
  assert_eq!(runtime.balances.balance(&"bob".to_string()), 20);   // bob has 20 balance
}
```

Now, when the code is run on terminal:
  
```bash
$ cargo r
Error: Insufficient funds.
```

Basically, there is no panic. The error is swallowed.

### Error propagation `?` vs `expect()`

`?` is used to propagate error to the caller function, while `expect()` is used for debugging purpose.

Suppose, we have a function `get_age()` which returns age of a person. But, if the person is not found in database, it should return an error. So, we can do like this:

```rust
fn get_age() -> Result<u8, &'static str> {
  // Here, error propagation is used to return error to the caller function.
  let db = Database::new();
  let age = db.get_age()?;
  Ok(age)
}

fn main() -> Result<(), &'static str> {
  // Here, `expect` is used to panic if error occurs, because we want that to happen.
  let age = get_age().expect("Person not found in database");
  println!("Age is {}", age);
  Ok(())
}
```

## External crates

### eyre

- It is a fork of `anyhow`.
- In place of `std::Result<String, &'static str>`, we can use `eyre::Result<String>`.
- For Ok, `Ok("Rust is beautiful")` can be used in both cases.
- And in place of `Err("Invalid input")`, we can use `Err(eyre::eyre!("Invalid input"))`.

## References

- [Rust: Error Handling](https://www.youtube.com/watch?v=y3wUCb-uS3g)
