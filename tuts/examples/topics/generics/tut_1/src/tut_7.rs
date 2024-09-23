/*
   Here, we show an example where there is single trait `Summary` as a function argument & then
   we implement for the struct `NewsArticle`.

   In the next eg, we would see multiple traits applied for a function argument using `+` sign.
*/

trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("Read more")
    }
}

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
    fn notify(item: &impl Summary) -> String {
        format!("Breaking news: {}", item.summarize_author())
    }

    // M-2: more verbose expression
    // fn notify<T: Summary>(item: &T) -> String {
    //     format!("Breaking news: {}", item.summarize_author())
    // }

    // =========****=========
    // If implemented for more arguments,
    // M-1
    fn notify(item1: &impl Summary, item2: &impl Summary) -> String {
        format!("Breaking news: {}", item.summarize_author())
    }

    // M-2
    fn notify<T: Summary>(item1: &T, item2: &T) -> String {
        format!("Breaking news: {}", item.summarize_author())
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
