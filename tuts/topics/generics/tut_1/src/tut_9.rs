/*
   impl Trait as function return type. So, here we don't need to mention the return type (especially when we don't know),
   rather the behaviour/trait they return type possess.

   NOTE: This function can't be defined inside a `impl` definition for a struct.

   There is a workaround for this via returning the trait's definition function type as `Self` like this:

    ```rs
    trait Summary {
        fn summarize_author(item: String) -> Self;
    }
    ```
*/

use std::fmt::{Debug, Display};

trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("Read more")
    }
}

#[derive(Debug)]
struct NewsArticle {
    author: String,
    heading: String,
    category: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        // implementation here
        format!(
            "The article with heading - \'{1}\' by \'{0}\' falls under the \'{2}\' category",
            self.author, self.heading, self.category
        )
    }
}

struct Tweet {
    author: String,
    heading: String,
    tag: String,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        // implementation here
        format!(
            "The tweet with heading - \'{1}\' by \'{0}\' falls under the \'{2}\' category",
            self.author, self.heading, self.category
        )
    }
}

// `impl Summary` trait implemented as function.
// Here, the function can return either `NewsArticle` or `Tweet`.
// Either works as both has implemented Summary trait.
fn returns_summarizable() -> impl Summary {
    NewsArticle {
        author: "Abhijit Roy".to_string(),
        heading: "SVB bank gets bankrupt".to_string(),
        category: "Finance".to_string(),
    }
}

// ❌ throws error because of incompatible types
fn returns_summarizable_on_condition(input: bool) -> impl Summary {
    if input {
        NewsArticle {
            author: "Abhijit Roy".to_string(),
            heading: "SVB bank gets bankrupt".to_string(),
            category: "Finance".to_string(),
        }
    } else {
        Tweet {
            author: "Ani".to_string(),
            heading: "Indian rupee becomes global with more than 40 countries".to_string(),
            tag: "finance".to_string(),
        }
    }
}

pub fn main() {
    let n1 = NewsArticle {
        author: String::from("Abhijit"),
        heading: "SVB bank has gone down by 60%".to_string(),
        category: "Finance".to_string(),
    };
}
