# Calculator

## About
* It's a calculator App based on CLI

## Build
* Check error w/o build: `$ cargo check`
* Compile optimized: `$ cargo build --release`

## Output
### Add
* Add `1` & `2.5`
```console
❯ cargo run -- 1 + 2.5
   Compiling calculator v0.1.0 (/Users/abhi3700/F/coding/github_repos/My_Learning-Rust/pro/calculator)
    Finished dev [unoptimized + debuginfo] target(s) in 0.37s
     Running `/Users/abhi3700/F/coding/github_repos/My_Learning-Rust/pro/calculator/target/debug/calculator 1 + 2.5`
Result: 3.5
```

### Mul
* Muliply `2` & `2.5` using `*`
```console
❯ cargo run -- 2 \* 2.5
     Running `/Users/abhi3700/F/coding/github_repos/My_Learning-Rust/pro/calculator/target/debug/calculator 2 '*' 2.5`
2 * 2.5 = 5
```
* Muliply `2` & `2.5` using `x`
```console
❯ cargo run -- 2 x 2.5
     Running `/Users/abhi3700/F/coding/github_repos/My_Learning-Rust/pro/calculator/target/debug/calculator 2 x 2.5`
2 x 2.5 = 0.8
```
* Muliply `2` & `2.5` using `X`
```console
❯ cargo run -- 2 X 2.5
     Running `/Users/abhi3700/F/coding/github_repos/My_Learning-Rust/pro/calculator/target/debug/calculator 2 X 2.5`
2 X 2.5 = 0.8
```

### Divide
* Divide `1` by `0` throws panic error as defined in the code.
```console
❯ cargo run -- 1 / 0  
    Finished dev [unoptimized + debuginfo] target(s) in 0.39s
     Running `target/debug/calculator 1 / 0`
thread 'main' panicked at 'Error: Divide by zero', src/main.rs:45:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### Error handling
* Run with number as oper. Throws error
```console
❯ cargo run -- 1 4 5
     Running `/Users/abhi3700/F/coding/github_repos/My_Learning-Rust/pro/calculator/target/debug/calculator 1 4 5`
thread 'main' panicked at 'Invalid operator used', src/main.rs:44:14
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
