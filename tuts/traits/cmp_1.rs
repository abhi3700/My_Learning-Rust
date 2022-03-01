/* 
    cmp traits like `Eq`, `PartialEq` for struct

    Here, the comparison is done b/w same type on LHS & RHS
*/
use std::cmp::PartialEq;

enum BookFormat {
    Paperback,
    Hardbook,
    Ebook
}

struct Book {
    isbn: u32,      // International Standard Book No.
    format: BookFormat,
}

impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        // M-1: only compare the isbn i.e. book no.
        // self.isbn == other.isbn

        // ------------------------------
        // M-2: only compare the format
        // self.format == other.format

        // ------------------------------
        // M-3: compare both the isbn & format
        self.isbn == other.isbn && self.format == other.format
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

    assert!(b1 == b2);

}