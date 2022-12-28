# Actix-web

## Overview

- Actix-web is a web framework for Rust.
- Faster than [Rocket](../rocket/).
- **Cons**:
  - Checked version `4` & found that any updation made on the code level doesn't get updated on the fly on the browser api response.
  - Doesn't show any logs on the console.
    ![](../../img/actix_hello_api.png)

## Getting Started

- Install Rust and Cargo.
- `$ cargo new hello-world`
- `$ cargo run`

## Lessons

- [x] [Hello World](./hello-world/src/l1_api.rs)
- [x] [Application](./hello-world/src/l2_app.rs)
      ![](../../img/actix_hello_app.png)
- [x] [Application State](hello-world/src/l3_app_state.rs)
      ![](../../img/actix_hello_app_state.png)

## References

- [Documentation](https://actix.rs/docs/)
