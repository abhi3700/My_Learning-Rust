# Borrowing & Ownership

## Overview

- <u>Borrow Checker</u>: You can move the data itself and give up ownership in the process, create a copy of the data and pass that along, or pass a reference to the data and retain ownership, letting the recipient borrow it for a while. The most appropriate approach depends entirely on the situation. Try [this](./tuts/functions/borrow_checker.rs)

- Rust memory safety is based on this rule: Given an object T, it is only possible to have one of the following:

  - Having several immutable references (&T) to the object (also known as aliasing).
  - Having one mutable reference (&mut T) to the object (also known as mutability).

    ```rs
    // ✅
    let x = 5;
    let y = &x;
    let z = &x

    // ❌
    let mut x = 5;
    let y = &mut x;
    let z = &mut x;
    ```
