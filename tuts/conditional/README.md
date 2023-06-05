# Conditional

## if-else, match

- In Rust, `match` is used more often than `if-else`. `match` is similar to `switch` in other languages and requires to cover all the cases.
- <u>Pattern matching</u>: `match` vs `if-let`:
  - The former has to cover all the cases
  - The latter is used for only 1 case, if we don't want to cover all the cases.
- #### `if-let` is preferred over `if` condition in cases of pattern matching with `Option`, `Result`

  ```rust
  let x = 3;
  if (x == 3) {
      println!("x is equal to 3");
  }
  ```

  ```rust
  let value = Some(3);
  if let Some(x) = value {
      println!("x is equal to 3");
  }
  ```

  The two expressions you've mentioned serve different purposes in Rust, so it's not really a matter of one being 'better' than the other; rather, it depends on the context of use.

  1. `if x == 3`: This is a simple comparison. If the value of x is equal to 3, then the code block following this if statement will execute. If x is not equal to 3, the code block will be skipped.

  2. `if let x = 3`: This is actually a misuse of Rust's if let construct. The if let statement is used for pattern matching, and it works a bit differently. The correct usage would be if let `PAT` = `EXPR`, where PAT is a pattern and `EXPR` is an expression. For example, you might use `if let Some(x) = some_option` to check if an Option is Some and, if so, bind the value inside to x.

  In your specific case, if you want to compare `x` to `3`, you should use `if x == 3`. If you need pattern matching (like checking if an `Option` is `Some` or a `Result` is `Ok`), then if let is the appropriate construct.

## while

## loop

## for

- `..` used for range like `1..4` i.e. 1, 2, 3. But, if `1..=4` i.e. 1, 2, 3, 4

## break

## continue
