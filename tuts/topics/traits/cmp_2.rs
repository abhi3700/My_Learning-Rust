/* 
    cmp traits like `Eq`, `PartialEq` for struct

    Here, comparison happens b/w 2 different types on either sides - LHS, RHS
*/
use std::cmp::PartialEq;

#[derive(PartialEq)]
enum BookFormat {
    Paperback,
    Hardbook,
    Ebook
}

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

}