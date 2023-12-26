## Containers in Rust

### Array

- Arrays are fixed length. They are stored in stack.
- E.g.

  ```rs
  let array: [i32; 4] = [42, 10, 5, 2];
  ```

  ![](../../img/array.png)

- Arrays `[T]` are **immutable** by default and **even with** `mut`, its element count cannot be changed.

> If you are looking for a dynamic/ growable array, you can use vectors. Vectors can contain any type of elements but all elements must be in the same data type.

Just like `&str` -> `String` similarly, `&[T]` -> `Vec<T>`

```rust
// conversion from array to vector
let arr = [1, 2, 3];
let vec = arr.to_vec();

// conversion from vector to array
let vec = vec![1, 2, 3];
let arr = vec.as_slice();
// OR
let arr = &vec[..];
```

---

**Array vs Slice**:

In Rust, both slices and arrays are used to store multiple values of the same type. However, they differ in their flexibility and use cases.

1. **Array**

   - An array in Rust has a fixed size, which is determined at compile time. You cannot grow or shrink an array after it is declared.
   - The size is part of the type of the array, so `[i32; 5]` and `[i32; 10]` are different types.
   - Here is an example of an array declaration: `let a: [i32; 5] = [1, 2, 3, 4, 5];`

2. **Slice**
   - A slice is a dynamically-sized view into a sequence of elements in an array.
   - Unlike arrays, slices don't need to have their size determined at compile time. They are flexible and can point to all or part of an array.
   - Slices have the type signature `&[T]`, where `T` can be any type.
   - Here is an example of a slice that points to a portion of an array: `let slice: &[i32] = &a[1..3];`

In the context of your Solana program, when we say `accounts: &[AccountInfo]`, we're saying that `accounts` is a slice of `AccountInfo` references. This means that `accounts` could contain any number of `AccountInfo` references, including zero. The function does not need to know at compile time how many accounts will be passed to it.

To recap, the main difference between slices and arrays in Rust is that arrays have a fixed size determined at compile time, while slices are dynamic and their size can change at runtime.

### Vector

- Vectors are dynamic length. They are stored in heap.
- They are allocated/deallocated based on the capacity of the vector filled.
- In this code:

  ```rs
  //! step-1
  let mut v: Vec<i32> = vec![1, 2, 3, 4];
  //prints 4
  println!("v's capacity is {}", v.capacity());
  println!("v's length is {}", v.len());  // -> 4
  println!("Address of v's first element: {:p}", &v[0]); //{:p} prints the address
  v.push(5);

  //! step-2
  //prints 8
  println!("v's capacity is {}", v.capacity());
  println!("v's length is {}", v.len());  // -> 5
  println!("Address of v's first element: {:p}", &v[0]);
  ```

  **At step-1**:

  ![](../../img/vector_memory.png)

  **At step-2**:

  ![](../../img/vector_memory2.png)

  > If you do not see a different address after pushing more elements onto a vector, it might be because the allocator had enough space at the end of the original buffer such that the new and the old buffers have the same starting address. Try pushing more elements and you will see a different address. Read about C library function `realloc` to understand how this might happen.

- One should use idiomatic patterns to get the job done:
  - `find()`: find the element & return the index
  - `map()`: loop over the elements & return a new vector with parsed `Fn`.
    - we can use chained map like this: `v.iter().map(|x| x * 2).map(|x| x + 1).collect::<Vec<i32>>();` as `map` returns an iterator.
  - `filter()`: loop over the elements & return a new vector with filtered elements based on `Fn`.
  - `fold()`: loop over the elements & return a new vector with folded elements based on `Fn`.
  - `enumerate()`: loop over the elements & return a new vector with zipped elements as (index, value).
  - `zip()`: with 2 vectors, loop over the elements & return a new vector with zipped elements as (element1, element2).
  - `unzip()`: with 1 vector of tuples, loop over the elements & return a new vector with unzipped elements as (vector1, vector2).

## BTreeMap

Why **BTreeMap** than **HashMap**?

- For sorted, range-based operations like getting all keys in a given range, **BTreeMap** is more efficient with `O(logN)`. Additionally, it stores the pointers for sorting the keys.
- For faster unsorted access, HashMap is more efficient with `O(1)`.

Basically, here in blockchain we would require frequent queries & might have to get the list for users. Hence, it’s better to use BtreeMap.

[Comparo with HashMap](https://chat.openai.com/share/ae449805-7a2e-4c5b-be1d-12b3f9f4d9fb).
