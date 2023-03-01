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

## Reference

- [Asynchronous Programming in Rust](https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html)
