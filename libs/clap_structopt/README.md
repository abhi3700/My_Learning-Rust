# CLI App

CLAP: **C**ommand **L**ine **A**rgument **P**arsing

## Overview

## Installation

Look at the [`Cargo.toml`](./Cargo.toml) file to view the dependencies used.

As of now, the following dependencies are used: `clap`, `structopt`.

## Build

```sh
$ cargo run
cli_app 0.1.0
A simple CLI app example using clap and structopt

USAGE:
    cli_app <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    add      Add two numbers
    hello    Say hello
    help     Prints this message or the help of the given subcommand(s)
```

## Usage

### Hello

- help hello:

```sh
$ cargo run -- hello --help
cli_app-hello 0.1.0
Say hello

USAGE:
    cli_app hello [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --name <name>    Who to say hello to [default: World]
```

- without setting any option:

```sh
$ cargo run -- hello
Hello, World!
```

- setting the option using long name:

```sh
$ cargo run -- hello --name Abhijit
Hello, Abhijit!
```

- setting the option using short name:

```sh
$ cargo run -- hello -a Abhijit
Hello, Abhijit!
```

### Add

- help add:

```sh
$ cargo run -- add --help
cli_app-add 0.1.0
Add two numbers

USAGE:
    cli_app add --num1 <num1> --num2 <num2>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --num1 <num1>    First number
    -c, --num2 <num2>    Second number
```

- setting the option using long name:

```sh
$ cargo run -- add --num1 1 --num2 4
The sum of 1 and 4 is 5
```

- setting the option using short name:

```sh
$ cargo run -- add -b 1 -c 4
The sum of 1 and 4 is 5
```

### Output

In order to view the program output with different cli args/params, just run the program using `cargo` like this:

```sh
❯ cargo run -- --funding-amount 5000
```

Here, `--` is used to separate the `cargo run` args from the cli args. It can also be seen for the `hello` and `add` subcommands shown above.
