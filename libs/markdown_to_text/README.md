# Markdown to Text Converter

This is a simple markdown to text converter using Rust.

## Overview

**Pros**:

- It's good. It's simple. It's fast.

**Cons**:

- The bullet points are not converted into same line. It pushes the sentence to next line.

## Usage

Add this to your `Cargo.toml`:

```sh
$ cargo add markdown_to_text
```

and in `main.rs` file:

```rust
use markdown_to_text::convert;

let markdown: String = [...];
let plain_text: String = markdown_to_text::convert(&markdown);
```

[Example](./demo/src/main.rs)
