# Actix-web

## Overview

- Actix-web is a web framework for Rust.
- Actix provides us 2 frameworks:
  - **Actix-web** (for web development i.e. REST API)
  - **Actix-actor** (for websockets mainly as per v4)
- Faster than [Rocket](../rocket/), but slower in comparison to Axum. Axum is not yet production ready, otherwise it would have been the best choice. In cases like smaller binary size, less memory usage, Axum is better than Actix-web.
- **Cons**:
  - Checked version `4` & found that any updation made on the code level doesn't get updated on the fly on the browser api response.
    > Solution: Use instead: `$ cargo watch -x run`
  - Doesn't show any logs on the console.
    ![](../../img/actix_hello_api.png)
    > Solution: Just add `env_logger` to show on the log based on per request basis.

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

In this analogy, the office building (server) houses multiple factories/departments (processes), and each factory has multiple workers (threads) working in it. Each worker (thread) can work independently, but they share the resources of their factory (process). Similarly, each factory (process) works independently but shares the resources of the office building (server).

---

A server can handle hundreds to thousands of processes at a time. Each process can handle hundreds to thousands of threads at a time. Each thread can handle one request at a time. So, a server can handle hundreds to thousands of requests at a time.

Now, the no. of processes & threads can be increased or decreased based on the load. This is called **scaling** (horizontal or vertical).

---

Now, when a server is spun up, it by default uses all the CPU cores i.e. CPU time (and required memory). We get to see on CLI (using `env_logger` + `actix-web`), how many threads are being used by the server. These threads are also called "Workers". We can limit the no. of threads for each server. This is called **thread-pooling**.

---

![](../../img/actix_web_server_to_threads_to_scopes_to_services.png)

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

---

## Coding

There are 4 main functions which can be composed in different ways (keeping `AppState` aside for a moment) while creating API routes:

- `service`: takes a `factory` argument inside
- `scope`: takes a `prefixed_path` argument inside which can be propagated with `route`
- `resource`: takes a `api_route_path` argument inside which can be propagated with `route`
- `route`: can have 2 or 1 arguments based on `scope` & `resource` respectively

And we can also leverage `Mutex` which essentially supports both `Send` & `Sync` that allows a data to flow between threads (concurrency) maintaining data consistency.

Actix-web RESTful API scaffoldings:

![](../../img/actix_web_restful_api_scaffoldings.png)

For better resolution, refer [this](../../img/rust_all.drawio)

---

For organization of `service`(s), we can have multiple patterns:

1. Put all scope based handlers in a single service

   ```rust
   HttpServer::new(move || {
           App::new().route("/home", web::get().to(home)).service(
               web::scope("/contracts")
                   .route("/sptoken/name", web::get().to(get_sptoken_name))
                   .route("/spnft/name", web::get().to(get_spnft_name))
                   .route("/rspnft/name", web::get().to(get_rspnft_name))
                   .app_data(sptoken_contract.clone())
                   .app_data(spnft_contract.clone())
                   .app_data(rspnft_contract.clone()),
           )
       })
       .bind(("127.0.0.1", port))?
       .run()
       .await
   ```

2. Put different services (NOT better than previous one)

   ```rust
   HttpServer::new(move || {
           App::new()
               .route("/home", web::get().to(home))
               .service(
                   web::scope("/contracts")
                       .route("/sptoken/name", web::get().to(get_sptoken_name))
                       .app_data(sptoken_contract.clone()),
               )
               .service(
                   web::scope("/contracts")
                       .route("/spnft/name", web::get().to(get_spnft_name))
                       .app_data(spnft_contract.clone()),
               )
               .service(
                   web::scope("/contracts")
                       .route("/rspnft/name", web::get().to(get_rspnft_name))
                       .app_data(rspnft_contract.clone()),
               )
       })
       .bind(("127.0.0.1", port))?
       .run()
       .await
   ```

---

There are 2 ways to create handler function:
**M-1**:

Personally, I prefer `M-1`, because we get to see all the API routes with the associated API handler functions in the `main()` function directly. Also, there is no need of creating a separate function called `config` in the handlers module.

> You can also use `scope` to make the APIs more modular via prefixing just like `namespace` (used in C++), `mod` (in Rust).

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

---

`scope` vs `resource`:

- The former refers to prefixing the API routes & also `route` function attached requires 2 arguments.

  ```rust
  HttpServer::new(|| {
      App::new().route("/index", web::get().to(index)).service(
          // prefixes all resources and routes attached to it...
          web::scope("/app")
              // ...so this handles requests for `GET /app/tasks` & `POST /app/tasks`
              .route("/tasks", web::get().to(get_task))
              .route("/tasks", web::put().to(create_task)),
      )
  })
  ```

- The latter refers to linking multiple routes (with 1 argument i.e. `HTTP_method.handler_func_name`) to a API route.

  ```rust
  HttpServer::new(|| {
      App::new().route("/index", web::get().to(index)).service(
          // prefixes all resources and routes attached to it...
          web::resource("/app/tasks")
              // ...so this handles requests for `GET /app/tasks` & `POST /app/tasks`
              .route(web::get().to(get_task))
              .route(web::put().to(create_task)),
      )
  })
  ```

- Now, the former & latter can be composed like this:

  ```rust
  HttpServer::new(|| {
      App::new().route("/index", web::get().to(index)).service(
          // prefixes all service (resources and routes) attached to it...
          web::scope("/app").service(
              web::resource("/tasks")
                  // ...so this handles requests for `GET /app/tasks` & `POST /app/tasks`
                  .route(web::get().to(get_task))
                  .route(web::put().to(create_task)),
          ),
      )
  })
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

## FAQs

<details>
<summary>
<b>Show</b>
</summary>

### What is the difference betweeen Actix & Actix Web? There are 2 inter-related crates, which one to use for RESTful API?

[Source](https://actix.rs/docs/whatis)

Actix and Actix Web are two separate but related crates in Rust:

1. **Actix**: Actix is an actor framework for Rust, which provides a foundation for building concurrent systems based on the actor model. However, as per the provided link, the usefulness of Actix as a general tool is diminishing as the futures and async/await ecosystem in Rust matures. Currently, the use of Actix is only required for WebSocket endpoints in Actix Web.

2. **Actix Web**: Actix Web, on the other hand, is a powerful and pragmatic web framework for Rust. It was originally built on top of the Actix actor framework, but now it's largely unrelated to the actor framework and is built using a different system. Actix Web provides the tools you need for handling HTTP requests and responses, routing requests to the appropriate handlers, managing middleware, and more. It supports HTTP/1 and HTTP/2, as well as TLS (HTTPS).

If you're building a RESTful API, you would typically use Actix Web. Actix Web provides the tools you need for handling HTTP requests and responses, routing requests to the appropriate handlers, managing middleware, and more. While Actix provides the underlying actor system that Actix Web uses, you typically don't need to interact with Actix directly when building a web application.

### As per the actix web, what is the difference between scope & service?

In Actix Web, `scope` and `service` are both used to organize routes, but they serve different purposes and have different behaviors:

1. **Scope**: The `scope` function in Actix Web is used to create a new scope for your routes, essentially creating a namespace. All routes defined within this scope will have a common path prefix. For example, if you define a scope with the path "/api", all routes within this scope will start with "/api". This is useful for grouping related routes together, such as all the routes related to a specific part of your application. Scopes can be nested, allowing you to create a hierarchical structure for your routes.

2. **Service**: The `service` function is used to register a route or a resource at a specific path. A service in Actix Web is a component that responds to requests. This could be a simple function, or it could be a more complex object that implements the `Service` trait. When you register a service, you're telling Actix Web to use that service to handle requests at the specified path.

In summary, `scope` is about organizing your routes and creating a hierarchical structure, while `service` is about handling requests at specific routes. You can use `service` within a `scope` to register services at the routes defined within that scope.

### In order to build websockets, do I have to use `actix-web` along with `actix`?

Yes, if you're planning to build WebSocket endpoints in Rust using the Actix ecosystem, you would typically use both `actix-web` and `actix`.

`actix-web` provides the web server framework, including request handling, routing, middleware, and more. However, `actix-web` itself does not directly provide WebSocket support.

The `actix` crate provides an actor framework, which is a model of concurrent computation where components (actors) communicate through asynchronous messages. This model is well-suited to WebSockets, which involve bidirectional communication between the client and the server.

In the Actix ecosystem, WebSocket support is provided by the `actix-web-actors` crate. This crate provides a WebSocket server implementation built on top of `actix-web` and `actix`. When you're using `actix-web-actors`, you're effectively using `actix-web` and `actix` together.

Here's a complete example:

```rust
use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

/// Define HTTP actor
struct MyWebSocket;

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

async fn ws_index(r: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let res = ws::start(MyWebSocket {}, &r, stream);
    res
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/ws/", web::get().to(ws_index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

In this example, `MyWebSocket` is a simple actor that echoes back any text or binary messages it receives, and responds to ping messages with a pong. This is just a basic example, and you would likely want to customize the behavior of your WebSocket actor to suit your needs.

Please note that you should add the necessary dependencies in your `Cargo.toml` file:

```toml
[dependencies]
actix-web = "4.0.0-beta.8"
actix-web-actors = "4.0.0-beta.8"
actix = "0.12.0"
```

The versions might change depending on the time you're implementing this. Always refer to the official documentation for the most accurate and up-to-date information.

### Does each API server would have only 1 app instance?

Reference: https://github.com/actix/actix-web

No! Each server instance can have many app instances. You can use scopes and services to organize your routes in order to have multiple app instances, but the app_state is shared across all scopes and services. Moreover, the `app_state` (with/without DB) is capable of handling multiple requests concurrently using `actix-web` (using Arc & Mutex / Connection pool respectively).

I tried doing like this, but it is wrong code:

❌:

```rust
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        AppVec(vec![
            App::new().route("/app1", web::get().to(greet)),
            App::new().route("/app2", web::get().to(farewell)),
        ])
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

Thought process is correct, but it throws compiler error related to trait bound failure.

So, it is advised to use `scope` and `service` to organize your routes in order to create multiple app instances in an API server.

### Q. Do we need to use `mutex` when using database for storage?

Given these:

- ✅ ORM (Diesel) library
- ✅ PostgreSQL DB
- ✅ Actix Web API framework
- ✅ mixed operations on the database (read and write)

When using Diesel ORM with Actix and PostgreSQL, you typically don't need to use a `Mutex` directly for managing your database connections. Diesel provides a connection pool (via the `r2d2` crate) that allows you to handle multiple database connections efficiently.

In Actix, you can store the Diesel connection pool in your application state. When a request comes in, you can get a connection from the pool and use it to interact with your database. This is thread-safe and doesn't require manual synchronization with a `Mutex`.

Here's a basic example of how you might set this up:

```rust
use actix_web::{web, App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set up database connection pool
    let manager = ConnectionManager::<PgConnection>::new("postgres://username:password@localhost/database");
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())  // Store connection pool in app state
            .route("/", web::get().to(some_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn some_handler(db: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>) -> String {
    // Get a database connection from the pool
    let conn = db.get().expect("Failed to get DB connection from pool");

    // Use the connection to interact with your database...

    "Response".to_string()
}
```

In this example, `some_handler` is an Actix handler function that gets a database connection from the pool and uses it to interact with the database. The `web::Data` extractor is used to get the connection pool from the application state.

Remember to replace `"postgres://username:password@localhost/database"` with your actual database URL.

This setup ensures that your database connections are managed efficiently and safely across multiple threads, without the need for manual synchronization with a `Mutex`.

### Q. In addition to last question, do we need to use `Arc` to the data (App State) struct?

No, as `app_data()` by defaults has Arc pointer. So, we don't need both the `Mutex` and `Arc` to the data struct in the app state as it is already thread-safe via connection pool when using Diesel ORM with Actix and PostgreSQL DB.

### Q. In the following code (correct), how does the routes know their individual `app_state` to refer to?

Code snippet:

```rust
HttpServer::new(move || {
        App::new().route("/home", web::get().to(home)).service(
            web::scope("/contracts")
                .route("/sptoken/name", web::get().to(get_sptoken_name))
                .route("/sptoken/symbol", web::get().to(get_sptoken_symbol))
                .route("/spnft/name", web::get().to(get_spnft_name))
                .route("/rspnft/name", web::get().to(get_rspnft_name))
                .app_data(Data::new(sptoken_contract.clone()))
                .app_data(Data::new(spnft_contract.clone()))
                .app_data(Data::new(rspnft_contract.clone())),
        )
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
```

In the provided `HttpServer` code, the routes within the `/contracts` scope can access the respective `app_data` using the `Data` extractor. When you use the `Data` extractor in your handler functions, actix-web will automatically search for the appropriate data based on the type of the data you are trying to extract.

For example, if you have the following routes and their respective handler functions:

```rust
.route("/sptoken/name", web::get().to(get_sptoken_name))
.route("/sptoken/symbol", web::get().to(get_sptoken_symbol))
```

And you have registered the `app_data` like this:

```rust
.app_data(Data::new(sptoken_contract.clone()))
```

In your handler functions, you can use the `Data` extractor to access the appropriate data based on the type:

```rust
async fn get_sptoken_name(sptoken_contract: Data<SPToken<EthersProviderHttp>>) -> impl Responder {
    // ...
}

async fn get_sptoken_symbol(sptoken_contract: Data<SPToken<EthersProviderHttp>>) -> impl Responder {
    // ...
}
```

In both handler functions, the `Data` extractor will search for the `app_data` that matches the type `SPToken<EthersProviderHttp>`. Since you have registered the `sptoken_contract` with this type, the correct data will be extracted and passed to the handler functions.

So, the routes will know which `app_data` to use based on the type specified in the `Data` extractor in the handler functions.

</details>

## References

- [Documentation](https://actix.rs/docs/)
