/*
Learnings:
==========
1.
    We get a compiler error with a lot of (useful) information. 
    The compiler tells us that we’re trying to assign the value from name to b after it had been moved to a. 
    The problem here is that, by the time we’re trying to assign the value of name to b, name doesn’t actually own the value anymore. 
    Why? Because ownership has been moved to a in the meantime.

2. This happens when `name` is assigned to `a`. Hence, expression `let b = name` will result in an error.
            [–– name ––] [––– a –––]
            +–––+–––+–––+–––+–––+–––+
stack frame │   │   │   │ • │ 8 │ 6 │ 
            +–––+–––+–––+–│–+–––+–––+
                          │
              +–––––––––––+
              │
            +–V–+–––+–––+–––+–––+–––+–––+–––+
       heap │ P │ a │ s │ c │ a │ l │   │   │
            +–––+–––+–––+–––+–––+–––+–––+–––+

SOURCE: https://blog.thoughtram.io/ownership-in-rust/

3.
let name = "Pascal".to_string();
let a = name;
let b = a.clone();

This results in duplicating "Pascal" for b. Now, the ownership is now with both b & a.
            [–– name ––] [––– a –––][–––– b ––––]
            +–––+–––+–––+–––+–––+–––+–––+–––+–––+
stack frame │   │   │   │ • │ 8 │ 6 │ • │ 8 │ 6 │
            +–––+–––+–––+–│–+–––+–––+–│–+–––+–––+
                          │           │
              +–––––––––––+           +–––––––+
              │                               │
            +–V–+–––+–––+–––+–––+–––+–––+–––+–V–+–––+–––+–––+–––+–––+–––+–––+
       heap │ P │ a │ s │ c │ a │ l │   │   │ P │ a │ s │ c │ a │ l │   │   │
            +–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+

4. Hence, we just need a reference of name & feed into a & b.
*/

pub fn greet(s: &String) {
    println!("Hello, {}", s);

}

pub fn run() {
    let name = "Pascal".to_string();
    let a = &name;
    greet(a);
    let b = &name;
    greet(b);
}