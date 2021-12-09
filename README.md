# My_Learning-Rust
Rust programming language

## Installation
### Linux or MacOS
* Install
```console
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

* Update using `$ rustup update`

## Commands
* View installed version via `$ rustup show`
* Check latest version via `$ rustup check`

>  Often, `cargo check` is much faster than `cargo build`, because it skips the step of producing an executable. If you’re continually checking your work while writing the code, using `cargo check` will speed up the process! As such, many Rustaceans run `cargo check` periodically as they write their program to make sure it compiles. Then they run `cargo build` when they’re ready to use the executable.

## Getting Started
### Code
```rs
fn main() {
	println!("Hello World!");
}
```

### Compile
```console
$ rustc hello.rs
```

### Output
```console
$ ./hello
```

## Concepts
> “Ownership is Rust’s most unique feature, and it enables Rust to make memory safety guarantees without needing a garbage collector.”

> <u>Borrow Checker</u>: You can move the data itself and give up ownership in the process, create a copy of the data and pass that along, or pass a reference to the data and retain ownership, letting the recipient borrow it for a while. The most appropriate approach depends entirely on the situation. Try [this](./tuts/functions/borrow_checker.rs)
> - Stack (fixed size like char, bool, int; less costly; quick to access by calling var like easy to copy the var) | Heap (variable size like string, list, class; more costly; access var or object via pointer)

* By default, all the variables are defined as `immutable` equivalent to `const` in JS/TS.
* The value of mutable variable can be changed, but not the type.

### Basics
#### Primitive types and Variables
1. Various sizes of integers, signed and unsigned (i32, u8, etc.)
1. Floating point types f32 and f64.
1. Booleans (bool)
1. Characters (char). Note these can represent unicode scalar values (i.e. beyond ASCII)

#### Print
* 1. formatting variables inside `println` function
```
let name = "Abhijit";
let age = 28;

println!("My name is {name}, and age is {age}");					// ❌
println!("My name is {0}, and age is {1}", name, age);		// ✔️
println!("My name is {}, and age is {}", name, age);			// ✔️
```
* 2. Multiple usage of variables without repetition
```
let alice = "Alice";
let bob = "Bob";

println!("{0}, this is {1}. {1}, this is {0}", alice, bob);
```


#### Variables


## Troubleshoot
### 1. warning: path statement with no effect
* _Cause_: there is a statement having no effect
* _Solution_: Assign the variable to `_`.

Before:
```rs
    let result = match grade {
        "A" => { println!("Excellent!"); },
        "B" => { println!("Great!"); },
        "C" => { println!("Good"); },
        "D" => { println!("You passed"); },
        "F" => { println!("Sorry, you failed"); },
        _ => { println!("Unknown Grade"); }
    };

    result;
```

After:
```rs
    let result = match grade {
        "A" => { println!("Excellent!"); },
        "B" => { println!("Great!"); },
        "C" => { println!("Good"); },
        "D" => { println!("You passed"); },
        "F" => { println!("Sorry, you failed"); },
        _ => { println!("Unknown Grade"); }
    };

    // result;             // warning: path statement with no effect, Solution --> assign to `_` 
    let _ = result;

```

## References
* [Rust by example](https://doc.rust-lang.org/stable/rust-by-example/)
* [Book: The Rust Programming Language](https://doc.rust-lang.org/book/)
* [Rust for Haskell Programmers!](https://mmhaskell.com/rust)
	- [Part 1: Basic Syntax](https://www.mmhaskell.com/rust/syntax)
	- [Part 2: Managing Memory](https://www.mmhaskell.com/rust/memory)
	- [Part 3: Data Types](https://www.mmhaskell.com/rust/data)
	- [Part 4: Cargo Package Manager](https://www.mmhaskell.com/rust/cargo)
	- [Part 5: Collections and Lifetimes](https://www.mmhaskell.com/rust/lifetimes)
* [Learn Rust Documentation](https://learning-rust.github.io/)
* Rustlings: [Github repo](https://github.com/rust-lang/rustlings)

### Blogs
* [What is Rust and why is it so popular?](https://stackoverflow.blog/2020/01/20/what-is-rust-and-why-is-it-so-popular/)
* [Understanding the Rust borrow checker](https://blog.logrocket.com/introducing-the-rust-borrow-checker/)
* [No auto type deduction for function, but for local variable](https://stackoverflow.com/questions/24977365/differences-in-type-inference-for-closures-and-functions-in-rust)