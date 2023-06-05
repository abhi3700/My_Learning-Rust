# Modules

## Overview

- any folder can have a `mod.rs` which is going to be like `index.js` in JS.
- tree structure is as follows:

```bash
src/
├── main.rs
└── something
    ├── a.rs
    ├── b.rs
    └── mod.rs
```

When `Rust` check for `something.rs` and doesn't find it will check for the folder `something/` and then inside look for a file named `mod.rs`.

We can call `a` like this in the `main.rs`:

```rs
use crate::something::a::*;
use crate::something::b::*;
```

> When using `crate`, no need to use `mod` keyword in the `src/main.rs` file.

---

**Another example**:

In this rust project `src/` folder structure:

```sh
├── src
│   ├── app
│   │   └── README.md
│   ├── handlers.rs
│   ├── main.rs
│   ├── models.rs
│   └── utils
│       ├── determine_emoji.rs
│       ├── get_current_time.rs
│       └── mod.rs
```

When using `determine_emoji` inside `handler.rs` file:

```rust
use crate::utils::{determine_emoji, get_current_time};
```

provided the below is maintained:

```rust
// src/utils/mod.rs
pub mod determine_emoji;
pub mod get_current_time;
```

```rust
// src/main.rs
mod utils; // Add this line
```
