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

- `Option` vs `Result`

| Option                                                                                                                                                                                  | Result                                                                                                                             |
| --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| Some or None                                                                                                                                                                            | Ok or Err                                                                                                                          |
| An optional value can have either Some value or no value/ None.                                                                                                                         | A result can represent either success/ Ok or failure/ Err                                                                          |
| The Option type is a way to use Rust’s type system to express the possibility of absence                                                                                                | Result expresses the possibility of error                                                                                          |
| mainly used for var, function output. For struct, the parameters can have Option type. E.g. In full name, middle_name can be missing for cases, so define `middle_name: Option<String>` | mainly used for operation, function. As normally a variable won't have Err unless there is some calculation involved with this var |
| Don't want to print the exact issue as `None` doesn't have anything as param unlike `Some(T)`                                                                                           | Want to print the exact issue as `Err(E)` contains the message inside                                                              |
| E.g. "./tuts/error_handling/opt"                                                                                                                                                        | E.g. "./tuts/error_handling/res"                                                                                                   |

## References

- [Rust: Error Handling](https://www.youtube.com/watch?v=y3wUCb-uS3g)
