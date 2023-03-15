/*
   Continuing from `tut_9`, the bug is fixed here via providing different types as return from a function.

   For more, learn from ch-17 in Rust official book.
*/

trait Summary {}

struct NewsArticle {}
impl Summary for NewsArticle {}

struct Tweet {}
impl Summary for Tweet {}

// So, from the previous eg, 2 different types can be returned via wrapping inside a Box pointer.
fn returns_summarizable_on_condition(input: bool) -> Box<dyn Summary> {
    if input {
        Box::new(NewsArticle {})
    } else {
        Box::new(Tweet {})
    }
}

pub fn main() {}
