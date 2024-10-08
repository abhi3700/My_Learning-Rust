//! Separate name & urls of papers from the file "paper_urls.txt"
//!
//! Example:
//! - Impact of the global chip shortage on the development of in-memory chips: https://www.nature.com/articles/s41467-022-31598-5

use std::{fs::File, io::BufRead};

pub fn main() {
    // give the relative path of the file w.r.t. the `./src/main.rs` file
    let file = File::open("./tuts/fileio/paper_urls.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let mut names: Vec<String> = Vec::new();
    let mut urls: Vec<String> = Vec::new();

    // Each line contains a name & url separated by double : (colon)
    // So, we split each line into 3 parts & store them in the respective vectors
    // The first part is the name of the paper & the third part is the url of the paper
    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(":").collect();

        // remove the first 2 characters from the name (which is a hyphen & a space)
        names.push(parts[0][2..].to_string());

        // prefix https to the url
        urls.push(format!("https:{}", parts[2].to_string()));
    }

    names.iter().for_each(|name| println!("{}", name)); // print names of papers line by line
    urls.iter().for_each(|url| println!("{}", url)); // print urls of papers line by line
}
