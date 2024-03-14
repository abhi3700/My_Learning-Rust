---
title: Time
---

It is very important to understand how time works in Rust.

Currently, afaik there are 2 modules in Rust:

- std | `std::time::Instant`: returns timestamp.
- external crate | `chrono::now()`: returns `DateTime`, `Time`, `TimeZone` etc.

## `std::time::Instant`

**Code for current timestamp**:

```rust
use std::time::SystemTIme;

fn main() {
    println!(
        "now timestamp: {:?}",
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
    );
}
```

---

**Code for time elapsed in doing something**:

```rust
use std::time::Instant;

fn main() {
    let now = Instant::now();
    // run some function (long one preferably)
    std::thread::sleep(std::time::Duration::from_secs(5));
    let elapsed = now.elapsed().as_millis();    // in milliseconds
    println!("now: {:?}", elapsed);
}
```

---

**Cons**:

- It doesn't have Serde support. Hence, any struct where `Instant` is used, cannot be serialized.

e.g. this struct won't work ❌:

```rust
use std::time::Instant;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Person {
    id: u16,
    name: String,
    age: u8,
    work: String,
    went_to_work: Instant,
}
```

Error:

```text
error[E0277]: the trait bound `Instant: Deserialize<'_>` is not satisfied
    --> src/type_struct.rs:20:19
     |
20   |     went_to_work: Instant,
     |                   ^^^^^^^ the trait `Deserialize<'_>` is not implemented for `Instant`
     |
```

Hence, the correct ✅ struct is:

```rust
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Person {
    id: u16,
    name: String,
    age: u8,
    work: String,
    went_to_work: i64,
}
```

**Q. Now, how to set the timestamp**?

We can use `chrono` crate for this purpose. See below code snippet:

```rust
use chrono::Utc;

fn main() {
    let p1 = Person {
        id: 1,
        name: "Abhi".to_string(),
        age: 30,
        work: "Developer".to_string(),
        went_to_work: Utc::now().timestamp(),
    };
}
```

Well chrono requires additional crate to be addded in `Cargo.toml`:

```toml
[dependencies]
# cargo add chrono
chrono = "0.4.26"
```
