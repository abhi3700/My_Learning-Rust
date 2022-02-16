/* 
    Learning
    - understanding lifetime
    - wrap <'a> around App struct & config variable, as it is using Config

fn some_function(val: &i32) {
    ...
}

is expanded to this:
fn some_function<'a>(val: &'a i32) {
    ...
}

    - lifetimes are a compile-time feature and don’t exist at runtime

OBSERVATION
===========
You might be wondering: Why can’t the compiler simply expand our types with lifetimes just like it does with functions?

Good question! Turns out, earlier versions of the compiler actually did exactly that. However, developers found that part confusing and preferred it to know exactly when one value borrows something from another.

One last thing to note here, if App was borrowed in another type, that type will have to define its lifetime parameters as well:
*/


struct Config {
    
}

struct App<'a> {
    config: &'a Config
}

struct Platform<'a> {
    app: App<'a>
}

pub fn run() {
    
}