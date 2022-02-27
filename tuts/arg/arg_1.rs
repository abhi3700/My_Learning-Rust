/*
    NOTE: `unwrap()` return the value into `first` instead of Some()
*/

use std::env::{args, Args};


pub fn run() {
    let mut args = args();
    println!("{:?}", args);

    // let first = args.nth(1);       // Some("freecodecamp")


    // Case-1 with `$ cargo run -- 1 + 2 3`
    // let first = args.nth(0).unwrap();       // When run 1st, output -> "target/debug/my_learning_rust"
    // let second = args.nth(2).unwrap();      // "2", bcoz it iterates from '1'. Hence, the array [1, +, 2]. So, 2nd index is '2'
    
    // Case-2 with `$ cargo run -- 1 + 2 3`
    // let first = args.nth(1).unwrap();       // When run 1st, output -> "1" 
    // let second = args.nth(2).unwrap();      // output -> "3", bcoz it iterates from '+'. Hence, the array [+, 2, 3]. So, 2nd index is '3'
    
    // Case-3 with `$ cargo run -- 1 + 2`. Here, target -> 1 + 2
    let first = args.nth(1).unwrap();       // When run 1st, output -> "1" 
    let oper = args.nth(0).unwrap();      // output -> "3", bcoz it iterates from '+'. Hence, the array [+, 2, 3]. So, 2nd index is '3'
    let second = args.nth(0).unwrap();      // output -> "3", bcoz it iterates from '+'. Hence, the array [+, 2, 3]. So, 2nd index is '3'
    
    println!("{:?}", first);
    println!("{:?}", oper);
    println!("{:?}", second);
}

/* 
Execute:
========
- `$ cargo run`
```
Args { inner: ["target/debug/my_learning_rust"] }
```

- `$ cargo run -- freecodecamp`
```
Args { inner: ["target/debug/my_learning_rust", "freecodecamp"] }
```

- `$ cargo run -- freecodecamp roy`
```
Args { inner: ["target/debug/my_learning_rust", "freecodecamp", "roy"] }
```
*/