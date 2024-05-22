# `indoc` crate

## Description

- It allows you to write indented documentation in your Rust code for a product.

## Usage

```rust
use indoc::indoc;

fn main() {
    let testing = indoc! {"
        def hello():
            print('Hello, world!')

        hello()
    "};
    let expected = "def hello():\n    print('Hello, world!')\n\nhello()\n";
    assert_eq!(testing, expected);
}
```

Basically, adds `\n` & keeps the indentation after `\n` so that it appears just like the original string.
