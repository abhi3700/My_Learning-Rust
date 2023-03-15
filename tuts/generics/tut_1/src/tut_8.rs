/*
   multi-traits implemented as function argument type using `+` sign. 3 ways to represent

   Go for 'M-3'
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

impl NewsArticle {
    // M-1: with less expression [RECOMMENDED]
    fn notify(item: &(impl Summary + Debug)) -> String {
        format!("Breaking news: {}", item.summarize_author())
    }

    // M-2: more verbose expression
    // fn notify<T: Summary + Display>(item: &T) -> String {
    //     format!("Breaking news: {}", item.summarize_author())
    // }

    // Another eg:
    // M-1
    // fn notify_multi(item1: &(impl Summary + Display), item2: &(impl Clone + Debug)) -> String {
    //     format!("Breaking news: {} {:?}", item1.summarize_author(), item2)
    // }

    // M-2
    // fn notify_multi<T: Summary + Display, U: Clone + Debug>(item1: &T, item2: &U) -> String {
    //     format!("Breaking news: {} {:?}", item1.summarize_author(), item2)
    // }

    // M-3 [RECOMMENDED]
    fn notify_multi<T, U>(item1: &T, item2: &U) -> String
    where
        T: Summary + Display,
        U: Clone + Debug,
    {
        format!("Breaking news: {} {:?}", item1.summarize_author(), item2)
    }
}

pub fn main() {
    let n1 = NewsArticle {
        author: String::from("Abhijit"),
        heading: "SVB bank has gone down by 60%".to_string(),
        category: "Finance".to_string(),
    };

    println!("{}", NewsArticle::notify(&n1));
}
