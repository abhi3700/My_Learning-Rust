# Async

## Overview

- Asynchronous programming in Rust.
- Async is different than parallelism, concurrency.
  - **parallelism**: multiple tasks are running at the same time
  - **concurrency**: multiple tasks are running at the same time, but only one task is running at a time
  - **async**: multiple tasks are running at the same time, but only one task is running at a time, and the tasks are not blocking the main thread
- The async/await syntax is a way to write asynchronous code that looks like synchronous code.
- There is a `Future` trait in the standard library which is similar to the concept of a `Promise` in JavaScript.
- Futures are inert in Rust and make progress only when polled. Dropping a future stops it from making further progress.
- Async is zero-cost in Rust, which means that you only pay for what you use. Specifically, you can use async without heap allocations and dynamic dispatch, which is great for performance! This also lets you use async in constrained environments, such as embedded systems.
- **No built-in runtime** is provided by Rust. Instead, runtimes are provided by community maintained crates like `tokio`.
- Both single- and multithreaded runtimes are available in Rust, which have different strengths and weaknesses.

More on this in the [Async vs threads in Rust](https://rust-lang.github.io/async-book/01_getting_started/02_why_async.html#async-vs-threads-in-rust) section.

- On a last note, asynchronous programming is not better than threads, but different. If you don't need async for performance reasons, threads can often be the simpler alternative.

## Concepts

There is a crate - `tokio` which provides a runtime for executing asynchronous tasks like this:

```rust
#[tokio::main]
async fn main() {
   let mut counter = 0;
   let mut interval = tokio::time::interval(Duration::from_secs(1));
   loop {
      interval.tick().await;
      counter += 1;
      println!("tick {}", counter);
   }
}
```

---

There is a way to use single thread via `#[tokio::main(flavor = "current_thread")]`:

In eg. [`async_1`](./async_1/), this is going to be the output, when 50ms of sleep was put in a function

```console
[0] I'm a async function
[1] I'm a async function
[0]First result: DB read
[1]First result: DB read
[0] Second result: DB read
[1] Second result: DB read
```

But, in the same example if we remove the single thread flavor, then for different sleep time, the output is going to be different.

for 50 ms,

```console
[0] I'm a async function
[1] I'm a async function
[1]First result: DB read
[0]First result: DB read
[0] Second result: DB read
[1] Second result: DB read
```

### Rc & Arc

`Rc` stands for "reference counting" and is a non-atomic reference-counting smart pointer. It keeps track of the number of references to the data and deallocates the data when the last reference is dropped. `Rc` is useful when you need to share ownership of a value within a single thread.

`Arc` stands for "atomic reference counting" and is a thread-safe reference-counting smart pointer. It provides the same functionality as `Rc`, but it can be shared across multiple threads. `Arc` uses atomic operations to keep track of the number of references to the data, which makes it safe to use in a concurrent environment.

In the code snippet you provided, `Arc` is used to share ownership of the `provider` variable across multiple threads.

```rust
#[derive(Clone, Debug)]
pub struct Provider<P> {
    // ...
    node_client: Arc<Mutex<Option<NodeClient>>>,
}
```

Here, `Arc<Mutex<Option<NodeClient>>>` is a type that represents a shared ownership smart pointer (`Arc`) to a mutable lock (`Mutex`) that guards an optional value of type `NodeClient`. The `Mutex` is used to ensure that only one thread can access the `NodeClient` at a time.

This is necessary because the `tokio` library is used for asynchronous programming, which means that multiple tasks can run concurrently on different threads. By using `Arc`, each task can have a reference to the same `provider` variable, and the variable will be deallocated only when the last reference is dropped.

---

In both these shared pointers, data can be cloned in 2 methods:

```rust
//! Shown eg for Arc. Similarly, Rc pointer can also be cloned.
let data = Arc::new(5);
// M-1
let cloned_data = data.clone();
// M-2
let cloned_data = Arc::clone(&data);
```

## Threads

- `std::thread` module provides the `spawn` function for creating a new thread.
- `spawn` function takes a closure as an argument, and the closure is executed in the new thread.
- `spawn` function returns a `JoinHandle` which can be used to wait for the thread to finish.
- `JoinHandle` implements the `JoinHandle::join` method which waits for the thread to finish and returns a `Result` containing the return value of the closure.

So, the threads are run like this:

Threads run in the order they are spawned

```rust
let thread1 = thread::spawn(|| {
   println!("I'm a thread 1");
}).join();

let thread2 = thread::spawn(|| {
   println!("I'm a thread 2");
}).join();

let thread3 = thread::spawn(|| {
   println!("I'm a thread 3");
}).join();
```

But, this approach has limitation in cases where I want to set the order as 1-3-2. Then, I have to move the code. But, imagine those are in different files or modules. Then, it is not possible to move the code. So, this approach is not practical.

So, in order to make the threads run asynchronously, code like this:

```rust
let thread1 = thread::spawn(|| {
   println!("I'm a thread 1");
});

let thread2 = thread::spawn(|| {
   println!("I'm a thread 2");
});

let thread3 = thread::spawn(|| {
   println!("I'm a thread 3");
});

let _ = thread1.join();
let _ = thread2.join();
let _ = thread3.join();
```

[Code example](./sync_4.rs)

But, here it is not guaranteed that the threads will run in the order they are spawned. Not even the order of the `join` calls.

Now, to set the order, we can use synchronization primitives like `Mutex`, `Condvar`, and `Barrier`. These primitives allow you to coordinate the execution of multiple threads and ensure that they execute in a specific order.

TODO: Add more on this.

## FAQs

**Q: Which declarative macros like println! are used for automatically calling the Deref trait for Rc smart pointer?**

A: The declarative macro that is used for automatically calling the `Deref` trait for `Rc` smart pointers is `format!`. When you use `format!` with an `Rc<T>` argument, the `Deref` trait is automatically called on the `Rc<T>`, which returns a reference to the value inside the `Rc<T>`. This reference is then used to format the string.

**Q: In Rust, how can I access the value of an `Rc` smart pointer without using the dereference operator `*`?**

A: You can access the value of an `Rc` smart pointer without using the dereference operator `*` by using the `Deref` trait. When you use an `Rc<T>` as an argument to a function or macro that expects a reference to `T`, the `Deref` trait is automatically called on the `Rc<T>`, which returns a reference to the value inside the `Rc<T>`. This reference can then be used as if it were a reference to `T`. Examples of functions and macros that automatically call `Deref` include `println!`, `format!`.

**Q: What is the difference between `Rc` and `Arc`?**
**`Arc` vs `Rc`**: `Rc` and `Arc` are both smart pointers in Rust that allow multiple ownership of the same data. The main difference between them is that `Rc` provides shared ownership of a value within a single thread, while `Arc` provides shared ownership of a value across multiple threads.

## Reference

- [Asynchronous Programming in Rust](https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html)
