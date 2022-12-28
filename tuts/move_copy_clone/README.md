# Move | Copy | Clone

## Overview

In this tutorial we will learn how to `move`, `copy` and `clone` in rust.

## Move

- Generally seen in assignment of variable to the assigned variable.
- **Shallow copy** of the data. This means irrespective of what it is present inside the variable (`struct`, `enum`), it is going to be **moved** to the assigned variable.
- In the following code:

```rs
let v: Vec<i32> = vec![1, 2, 3];
let v2 = v;
```

This is what happens at the memory level:

From:

![](../../img/move_from.png)

To:

![](../../img/move_to.png)

- Now, this is what happens in the memory when we try to access `v` after the assignment in this code:

  ```rs
  let v: Vec<i32> = vec![1, 2, 3];
  let v2 = v;
  println!("v is {:?}", v); //error: use of moved value: `v`
  ```

  Rust lang prevents from situations like this:

  ![](../../img/move_error.png)

## Copy

- It is implicitly implemented for all the primitive data types like `i32`, `f32`, `bool`, `char` etc.
- In this code:

  ```rs
  let v: i32 = 42;
  let v1 = v;
  println!("v is {}", v);//compiles fine, no error!
  ```

  The following happens at the memory level:

  ![](../../img/copy.png)

- Without `clone`, it is not possible. As `Clone` is a super-trait of `Copy` used like this:

  ```rs
  pub trait Copy: Clone {}
  ```

  So, always use `Clone` when you use `Copy` when using `derive` macro.

- `Copy` is a marker trait, which means it does not have any methods. It is used to indicate that the type can be copied.
- There are few data types for which this trait is not applicable like `Vec`, `String`, `HashMap` etc.

  ```rs
  // error: the trait `Copy` may not be implemented for this type
  // label: this field does not implement `Copy`
  #[derive(Debug, Clone, Copy)]
  struct Team {
      players: Vec<String>,
  }
  ```

  Here, this error is because `Vec` does not implement `Copy` trait.

## Clone

- **Deep copy** of the data. This means that the data is copied recursively like in a struct, enum, etc. We can use `clone` method to do this.
- Looks like this:

![](../../img/clone.png)

## References

- [Moves, copies and clones in Rust](https://hashrust.com/blog/moves-copies-and-clones-in-rust/) ✅
- [Arrays, vectors and slices in Rust](https://hashrust.com/blog/arrays-vectors-and-slices-in-rust/) ✅
