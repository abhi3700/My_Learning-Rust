# Actix-web

## Overview

- Actix-web is a web framework for Rust.
- Actix provides us 2 frameworks:
  - **Actix-web** (for web development i.e. REST API)
  - **Actix-actor** (for websockets mainly as per v4)
- Faster than [Rocket](../rocket/), but slower in comparison to Axum. Axum is not yet production ready, otherwise it would have been the best choice. In cases like smaller binary size, less memory usage, Axum is better than Actix-web.
- **Cons**:
  - Checked version `4` & found that any updation made on the code level doesn't get updated on the fly on the browser api response.
  - Doesn't show any logs on the console.
    ![](../../img/actix_hello_api.png)
    > There is a solution to this. Just add `env_logger`.

---

Before diving into `actix-web`, it's important to understand the difference between server, process, thread. So, here it is:

1. **Server** 🏢: Think of a server as a big office building. It's a physical entity that houses various resources. Just like an office building has different departments (like HR, Finance, Marketing), a server has different applications running on it. Each application can be thought of as a department in this office building.

2. **Process** 🏭: A process can be thought of as a factory within the office building (server). Each factory (process) has its own resources and is independent. Just like a factory has its own workers, machinery, and power supply, a process has its own memory, data, and executes a sequence of instructions. Multiple factories (processes) can run in the same office building (server).

3. **Thread** 👥: A thread is like a worker in the factory (process). Just like multiple workers in a factory can work on different tasks simultaneously, multiple threads in a process can execute different parts of the program simultaneously. All workers (threads) in a factory (process) share the factory's resources.

Here's a visual representation:

```
🏢 Server (Office Building)
|
|--- 🏭 Process 1 (Factory)
|    |
|    |--- 👥 Thread 1.1 (Worker)
|    |--- 👥 Thread 1.2 (Worker)
|    |--- 👥 Thread 1.3 (Worker)
|
|--- 🏭 Process 2 (Factory)
     |
     |--- 👥 Thread 2.1 (Worker)
     |--- 👥 Thread 2.2 (Worker)
```

In this analogy, the office building (server) houses multiple factories (processes), and each factory has multiple workers (threads) working in it. Each worker (thread) can work independently, but they share the resources of their factory (process). Similarly, each factory (process) works independently but shares the resources of the office building (server).

---

A server can handle hundreds to thousands of processes at a time. Each process can handle hundreds to thousands of threads at a time. Each thread can handle one request at a time. So, a server can handle hundreds to thousands of requests at a time.

Now, the no. of processes & threads can be increased or decreased based on the load. This is called **scaling** (horizontal or vertical).

---

Now, when a server is spun up, it by default uses all the CPU cores i.e. CPU time (and required memory). We get to see on CLI (using `env_logger`), how many threads are being used by the server. These threads are also called Workers. We can limit the no. of threads for each server. This is called **thread-pooling**.

## Getting Started

- Install Rust and Cargo.
- `$ cargo new hello-world`
- `$ cargo run`

## Lessons

Inside the [demo](./demo/) example, there are multiple design patterns implemented. Each design pattern is a lesson in itself. Here are the lessons:

- [x] [Hello World](./hello-world/src/l1_api.rs)
- [x] [Application](./hello-world/src/l2_app.rs)
      ![](../../img/actix_hello_app.png)
- [x] [Application State M-1](hello-world/src/l3_app_state_local.rs)
      ![](../../img/actix_hello_app_state.png)
- [x] [Application State M-2](hello-world/src/l3_app_state_local.rs)
- []

---

For more comprehensive example, refer to these examples:

- [`REST API using actix + actix_web + diesel + postgresqlDB`](../databases/pgsql/demo/)

## Coding

There are 2 ways to create handler function:

**M-1**:

I prefer this, because we get to see all the API routes with the API handler functions in the `main()` function. Also, there is no need of creating a separate function called `config` in the handlers module.

```rust
// handlers.rs
pub async fn get_tasks(task_map: web::Data<Mutex<HashMap<u64, Task>>>) -> impl Responder {
    let map = task_map.lock().unwrap();
    HttpResponse::Ok().json(map.clone())
}
```

```rs
// src/main.rs
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // some code

    HttpServer::new(move || {
        App::new()
            .app_data(task_map.clone())
            .service(web::resource("/index").route(web::get().to(index)))
            .service(
                web::resource("/tasks")
                    .route(web::get().to(get_tasks))
                    .route(web::post().to(create_task)),
            )
            .service(web::resource("/tasks/{id}").route(web::put().to(update_task)))
            .service(web::resource("/tasks/{id}").route(web::delete().to(delete_task)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

**M-2**:

```rs
// handlers.rs
#[get("/tasks/{id}")]
pub async fn get_tasks(task_map: web::Data<Mutex<HashMap<u64, Task>>>) -> impl Responder {
    let map = task_map.lock().unwrap();
    HttpResponse::Ok().json(map.clone())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_tasks)
        // .service(create_entry_handler)
        // .service(update_entry_handler)
        // .service(delete_entry_handler)
        ;
}

```

```rs
// src/main.rs
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // some code

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(health_checker_handler)
            .configure(handlers::config)    // there needs to be a function which has this `config` function
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

}
```

## Run

In order to see the background activity & also the logs, follow the below steps:

1. `$ cargo add env_logger`
2. In `src/main.rs` file, add this:

   ```rust
   std::env::set_var("RUST_LOG", "debug");
   env_logger::init();
   ```

3. `$ cargo run`

   ```sh
       Finished dev [unoptimized + debuginfo] target(s) in 0.76s
       Running `target/debug/demo`
   [2023-05-20T11:46:00Z INFO  actix_server::builder] Starting 10 workers
   [2023-05-20T11:46:00Z INFO  actix_server::server] Actix runtime found; starting in Actix runtime
   ```

---

Also, `actix-web` doesn't show if the server is running on the required port or not. So, we need to add this code in the `main` function:

```rust
/// outside main.rs
pub(crate) fn is_port_available(port: u16) -> bool {
    match TcpListener::bind(("127.0.0.1", port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}


/// main.rs
let port = 8080;

if !is_port_available(port) {
    panic!("Port {} is already in use.", port);
}
```

## Troubleshoot

### 1. Address in use

- _Cause_: The port is already being used.
- _Solution_: change the port no. in the `main` function.

## References

- [Documentation](https://actix.rs/docs/)
