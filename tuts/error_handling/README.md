# Error

## About

## Coding

- Define custom errors like this:

```rs
#[derive(Debug)]
enum DoubleError {
    EmptyVec,
    // We will defer to the parse error implementation for their error.
    // Supplying extra info requires adding more data to the type.
    Parse(ParseIntError),
}
```

## References

- [Rust: Error Handling](https://www.youtube.com/watch?v=y3wUCb-uS3g)
