# Function

- `fn` keyword is used to define a function
- `->` is used to define the return type of the function
- `()` is used to define the arguments of the function
- `return` keyword is not used in Rust. The last line of the function is the return value of the function
- return type can be `Option`, `Result`.
- #### Use wildcard pattern `_` to ignore the value of the variable.

  In this code:

  ```rs
  fn add() -> (i32, i32, i32) {
      (1, 2, 3)
  }

  fn main() {
      let (x, _, _) = add();
      println!("x is {}", x);
  }
  ```

  The underscore `_` in Rust is called a wildcard pattern. It's used when you want to pattern match some parts of a data structure, but you're not interested in all parts of it. It's a way of saying "I know there's a value here, but I don't care about it".

  In your code `(x, _, _) = add();`, it means you're calling a function `add()` which returns a tuple of three values. You're interested in the first value, which is being assigned to x, but you're not interested in the second and third values, hence the `_` wildcard.

  The wildcard pattern is useful in many situations where you need to satisfy the compiler's requirement for exhaustive pattern matching, but there are some values you don't need to handle.

- [How a function works](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
- **Function vs Macro**
  - macro is a function which is built-in in Rust & is called by `!` at the end of the function name. We can also create custom macros for our use cases using declarative, procedural macros.
  - Both can be user defined.
  - Both can reduce the code duplication. But check for benchmark before making a decision related to pushing into production.
  - Syntax: `println!` calls the macro, but `println` is supposed to be defined first, otherwise it throws error.
