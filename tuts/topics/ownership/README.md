# Borrowing & Ownership

## Concepts

- <u>Borrow Checker</u>: You can move the data itself and give up ownership in the process, create a copy of the data and pass that along, or pass a reference to the data and retain ownership, letting the recipient borrow it for a while. The most appropriate approach depends entirely on the situation. Try [this](./tuts/functions/borrow_checker.rs)

- Rust memory safety is based on this rule: Given an object T, it is only possible to have one of the following:

  - Having several immutable references (&T) to the object (also known as aliasing).
  - Having one mutable reference (&mut T) to the object (also known as mutability).

    ```rust
    // ✅
    let x = 5;
    let y = &x;
    let z = &x

    // ❌
    let mut x = 5;
    let y = &mut x;
    let z = &mut x;
    ```

- Once immutably borrowed, one can't mutably borrow after that. Details on [E0506](https://doc.rust-lang.org/error_codes/E0506.html).

  ```rust
  let mut x = 42;

  // immutably borrowed
  let y = &x;

  x = 43; // ❌ error because of E0506 i.e. can't mutate as it is already borrowed.
  ```

  A solution would be to use into a separate function `print_y(&x)` like this & end its lifetime so that we can mutate `x` like this:

  ```rust
  fn print_y(x: &i32) -> &i32 {
      println!("y = {x}");
      x
  }

  fn main() {
      let mut x = 42;

      // immutably borrowed
      let y = print_y(&x);

      x = 43; // ✅ works.
  }
  ```

  **OR**

  ```rust
  fn main() {
      let mut x = 42;

      {
          // immutably borrowed
          let y = print_y(&x);
      }

      x = 43; // ✅ works.
  }
  ```
