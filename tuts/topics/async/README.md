# Asynchronous Rust

## Overview

- Asynchronous programming in Rust.
- Async is different than parallelism, concurrency.
  - **parallelism**: Multiple CPU cores doing multiple tasks at the same time. In rust, there is a crate called [`rayon`](https://crates.io/crates/rayon) which provides parallelism.
    ![](../../../img/parallelism_diagram.png)
  - **concurrency**: A single CPU core doing multiple tasks, but only one task is ning at a time. It's kind of like _faking parallelism_. It's like watching a YT video and listening to another music at the same time. You are not watching both at the same time, but you are switching between them. So, here the CPU does take some <kbd>pause</kbd>s in between. Hence, the idle time. So, these idle times could be because of moving some mouse or keyboard. Here, there is a concept of context switching.
    ![](../../../img/concurrency_diagram.png)
  - **asynchronous**: Single CPU core doing multiple tasks, but only one task is ning at a time, and the tasks are not blocking the main thread. Hence, _no idle time_. In the diagram below, the task-1 is paused because it is waiting for some resources/information that can be achieved after the completion of the other 3 tasks. So, after task 4 ends, the task-1 resumes. In rust, there is a crate called [`tokio`](https://crates.io/crates/tokio) which provides asynchronous programming. In other words, asynchronous involves sometimes parallelism or concurrency.
    ![](../../../img/async_diagram.png)
    And we should know when to use what. There are mainly 2 types of work:
    - **CPU bound**: lot of processing related work (cching no.s). In this case, parallelism can be really helpful.
    - **I/O bound**: lot of networking related work like connecting to a network server and waiting for its response. It could also be reading files or getting responses based on thousands of requests to a server.
- The async/await syntax is a way to write asynchronous code that looks like synchronous code.
- There is a `Future` trait in the standard library which is similar to the concept of a `Promise` in JavaScript.
- Futures are inert in Rust and make progress only when polled. Dropping a future stops it from making further progress.
- Async is zero-cost in Rust, which means that you only pay for what you use. Specifically, you can use async without heap allocations and dynamic dispatch, which is great for performance! This also lets you use async in constrained environments, such as embedded systems.
- **No built-in time** is provided by Rust. Instead, times are provided by community maintained crates like `tokio`.
- Both single- and multithreaded times are available in Rust, which have different strengths and weaknesses.

More on this in the [Async vs threads in Rust](https://rust-lang.github.io/async-book/01_getting_started/02_why_async.html#async-vs-threads-in-rust) section.

- On a last note, asynchronous programming is not better than threads, but different. If you don't need async for performance reasons, threads can often be the simpler alternative.

## Concepts

There is a crate - `tokio` which provides a time for executing asynchronous tasks like this:

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

`tokio` crate is the default choice in the industry for asynchronous tasks. It creates a thread per CPU core by default. So, if there are 4 cores, then 4 threads are created. And, the tasks are distributed among these threads by the time for multi-threading.

![](../../../img/tokio_multi_threading.png)

---

There is a trait called `Future` which acts as base layer for `tokio` crate. It is similar to the concept of a `Promise` in JavaScript. This is just to represent a type/process that implements `Future` trait which includes a function called `poll` which is used to check if a future has completed or not. If it has completed, then it returns `Poll::Ready` with the result. If it has not completed, then it returns `Poll::Pending`. Then, it might ask the time to check later (may be every few seconds). Or there can be a callback function that can be called when the future is ready.

Here is some piece of code that shows how `Future` trait is implemented:

```rust
use std::pin::Pin;
use std::task::{Context, Poll};

trait Future {
   // the computation will produce a result of type `Output` like u32, String, MyStruct, MyEnum, etc.
   type Output;

   // Pin is used to pin the memory location of the future. This is used to prevent the future from moving in memory.
   // Context type is used to pass information (whether the future is ready or not) to the future.
   fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

---

Behind any asynchronous request made using rust code, this is what is happening:

Here is the request/response code snippet:

```rust
use std::collections::Hashmap;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
   let resp = reqwest::get("https://www.rust-lang.org").await?.json::<Hashmap<String, String>>().await?;

   println!("{:#?}", resp);

   Ok(())
}
```

Below is the hardware diagram of the above code working under the hood:
![](../../../img/async_rust_1.png)

And here is the corresponding sequence diagram:

![](../../../img/async_rust_2.png)

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

This is necessary because the `tokio` library is used for asynchronous programming, which means that multiple tasks can concurrently on different threads. By using `Arc`, each task can have a reference to the same `provider` variable, and the variable will be deallocated only when the last reference is dropped.

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

So, the threads are like this:

Threads in the order they are spawned

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

So, in order to make the threads asynchronously, code like this:

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

But, here it is not guaranteed that the threads will in the order they are spawned. Not even the order of the `join` calls.

Now, to set the order, we can use synchronization primitives like `Mutex`, `Condvar`, and `Barrier`. These primitives allow you to coordinate the execution of multiple threads and ensure that they execute in a specific order.

TODO: Add more on this.

## FAQs

**Q: Which declarative macros like println! are used for automatically calling the Deref trait for Rc smart pointer?**

A: The declarative macro that is used for automatically calling the `Deref` trait for `Rc` smart pointers is `format!`. When you use `format!` with an `Rc<T>` argument, the `Deref` trait is automatically called on the `Rc<T>`, which returns a reference to the value inside the `Rc<T>`. This reference is then used to format the string.

**Q: In Rust, how can I access the value of an `Rc` smart pointer without using the dereference operator `*`?**

A: You can access the value of an `Rc` smart pointer without using the dereference operator `*` by using the `Deref` trait. When you use an `Rc<T>` as an argument to a function or macro that expects a reference to `T`, the `Deref` trait is automatically called on the `Rc<T>`, which returns a reference to the value inside the `Rc<T>`. This reference can then be used as if it were a reference to `T`. Examples of functions and macros that automatically call `Deref` include `println!`, `format!`.

**Q: What is the difference between `Rc` and `Arc`?**

A: **`Arc` vs `Rc`**: `Rc` and `Arc` are both smart pointers in Rust that allow multiple ownership of the same data. The main difference between them is that `Rc` provides shared ownership of a value within a single thread, while `Arc` provides shared ownership of a value across multiple threads.

## Reference

- [Asynchronous Programming in Rust](https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html)