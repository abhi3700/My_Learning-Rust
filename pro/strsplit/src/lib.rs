#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

#[derive(Debug)]
pub struct StrSplit {
    remainder: &str, // the remainder after splitting (left)
    delimiter: &str,
}

impl StrSplit {
    pub fn new(haystack: &str, delimiter: &str) -> Self {
        StrSplit {
            remainder: haystack,
            delimiter,
        }
    }
}

impl Iterator for StrSplit {
    type Item = &str;
    fn next(&mut self) -> Option<Self::Item> {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let haystack = "a b c d e";
        let letters = StrSplit::new(haystack, " ");
        assert_eq!(letters, vec!['a', 'b', 'c', 'd', 'e'].into_iter());
        // for letter in StrSplit::new(haystack, delimiter) {
        //     println!("{}", letter);
        // }
    }
}
