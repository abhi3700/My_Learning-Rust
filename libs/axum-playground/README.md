# Axum playground

## Usage

Run the examples with:

```sh
cargo r -r example <eg-file-name> -p axum-playground
# e.g. for `hello.rs`:
cargo r -r example hello -p axum-playground
```

## Output

It runs like this (on given socket address):

```sh
     Running `target/release/examples/hello`

```

And then to view the output, use `curl`:

```sh
❯ curl "localhost:3000/"
hello world
```

OR

Use [Thunder Client](https://marketplace.visualstudio.com/items?itemName=rangav.vscode-thunder-client) (VSCode extension) to view the output.
