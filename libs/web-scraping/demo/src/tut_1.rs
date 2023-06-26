//! Simple scrapping of a website

use reqwest::blocking::get;
use scraper::{Html, Selector};

pub(crate) fn retrieve_html(url: &String) -> String {
    let response = get(url).unwrap().text().unwrap();

    response
}

pub(crate) fn main() {
    let url = "https://news.ycombinator.com";

    // Get the html response from the url
    let html = retrieve_html(&url.to_string());
    // println!("{}", html);

    // parse the HTML document & get the doc_body
    let doc_body = Html::parse_document(&html);
    // println!("{:?}", doc_body);

    // select the elements with titleline class
    let title = Selector::parse(".titleline").unwrap();

    let mut count = 0;

    // iterate over the title elements in the doc_body
    for element in doc_body.select(&title) {
        // get the text from the element
        let text = element.text().collect::<Vec<_>>();
        println!("{:?}", text);
        count += 1;
    }

    println!("Total number of elements: {}", count);
}
