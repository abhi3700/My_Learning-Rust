/* 
    cmp traits like `Eq`, `PartialEq` for struct

    It's like `cmp_2.rs` except for adding `#[derive]` over Book struct for letting Book to book comparison.
    But, it's not possible like this unless we define what attributes (isbn, format) to be considered for comparison.

OBSERVATION:
============
By changing impl PartialEq for Book to impl PartialEq<BookFormat> for Book, we allow BookFormats to be compared with Books.

A comparison like the one above, which ignores some fields of the struct, can be dangerous. 
It can easily lead to an unintended violation of the requirements for a partial equivalence relation. 
For example, if we kept the above implementation of PartialEq<Book> for BookFormat and added an implementation of PartialEq<Book> for 
Book (either via a #[derive] or via the manual implementation from the first example) then the result would violate transitivity

*/
use std::cmp::PartialEq;

#[derive(PartialEq)]
enum BookFormat {
    Paperback,
    Hardbook,
    Ebook
}

#[derive(PartialEq)]
struct Book {
    isbn: u32,      // International Standard Book No.
    format: BookFormat,
}

// Implement <Book> == <BookFormat> comparisons
impl PartialEq<BookFormat> for Book {
    fn eq(&self, other: &BookFormat) -> bool {
        self.format == *other
    }
}

// Implement <BookFormat> == <Book> comparisons
impl PartialEq<Book> for BookFormat {
    fn eq(&self, other: &Book) -> bool {
        *self == other.format
    }
}

pub fn run() {
    let b1 = Book {
        isbn: 1,
        format: BookFormat::Paperback
    };

    let b2 = Book {
        isbn: 2,
        format: BookFormat::Paperback
    };

    // Implement <Book> == <BookFormat> comparisons
    assert!(b1 == BookFormat::Paperback);

    // Implement <BookFormat> == <Book> comparisons
    assert!(BookFormat::Paperback == b2);

    // The following should hold by transitivity but doesn't.
    assert!(b1 == b2);

}