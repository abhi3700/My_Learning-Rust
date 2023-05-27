# Diesel

## Overview

ORM for Postgres, MySQL, SQLite and others.

> for async version, use `diesel-async` crate.

> We also have [sqlx](https://crates.io/crates/sqlx) crate which is an async, pure Rust SQL crate featuring compile-time checked queries without a DSL. It is an alternative to ORM Diesel. This option is more flexible.

## Installation

## Integration with any project

```sh
$ cargo add diesel dotenv actix --features "diesel/postgres diesel/r2d2 diesel/chrono"
```

> Here, `dotenv`: for environment variables, `actix`: a web framework, `diesel`: for ORM, `diesel/postgres`: for postgres db, `diesel/r2d2`: for connection pooling, `diesel/chrono`: for date/time

```sh
# add for pgsql db
$ cargo install diesel_cli --no-default-features --features postgres
```

Use `.env` for connecting to a DB. So, set the `DATABASE_URL` as follows:

```sh
$ echo DATABASE_URL=postgresql://[USERNAME]:[PASSWORD]@[HOST]:[DB_PORT]/[DB_NAME] > .env
```

> Put `.env` in `.gitignore` file. Instead add `.env.example` file to the repo with all the environmennt variables with empty values

Now, we need to use `dotenvy` crate to load the `.env` file. So, add the following to `Cargo.toml`:

```sh
cargo add dotenvy
```

## Examples

- [REST API using actix + actix_web + diesel + postgresqlDB](../databases/pgsql/demo/)
  - Created a REST API using actix-web framework with diesel ORM and postgresql db & r2d2 connection pool, and dotenvy crate for environment variables (`.env` file). Also, used `serde` crate for serialization/deserialization of data. Also, used `chrono` crate for date/time. Also, used `actix` crate for implementing Actor model in Concurrency.

## CLI installation

```sh
# help with `diesel` commands like `$ diesel setup `
$ cargo install diesel_cli
# copy DB schema into rust structs
$ cargo install diesel_cli_ext
```

> diesel depends on `libpg` (for mac-arm), which can be installed using `brew` package manager.

```sh
$ diesel setup
```

## Coding concepts

The provided Diesel ORM schema represents a model of a database that deals with games, assets, companies, and users. Here's a breakdown of what each part of the schema does:

1. The schema starts by defining custom SQL types for various fields that will be used in the database. Each custom SQL type corresponds to a specific type in the PostgreSQL database system.

2. The `diesel::table!` macro is used to define representations of database tables in Rust. For each table, this macro defines a Rust struct with fields corresponding to the columns of the database table. The name of the struct is the same as the name of the table. The fields' names and types correspond to the columns' names and types in the table.

3. The `joinable!` macros specify how tables in the database are related to each other. This information is used by Diesel when you perform operations that involve more than one table, such as joining two tables together.

4. The `allow_tables_to_appear_in_same_query!` macro allows Diesel to know that multiple tables can be used in the same SQL query. This is necessary because Diesel's type system enforces that you can't perform certain operations on tables that aren't explicitly allowed to interact.

Here, `asset` is the name of the table. Inside the parentheses following the table name (`id` in this case) is the primary key of the table. Then, for each column of the table, there's a line of the form `column_name -> column_type`, which defines a field of the struct that represents a column of the table. The type may be a standard SQL type (like `Varchar` or `Int4`) or a custom type that was defined earlier (like `GameType` or `ChainType`). The `Nullable` wrapper indicates that the column can contain `NULL` values.

## Troubleshoot

### 1. note: ld: warning: ignoring file opt/anaconda3/lib/libpq.dylib, building for macOS-arm64 but attempting to link with file built for macOS-x86_64

- _Cause_: Because of `libpq.dylib` is not compatible with M1 chip (Apple Silicon) when installed using Anaconda (pre-installed). This happened when trying to install `diesel_cli` using `$ cargo install diesel_cli --no-default-features --features postgres`.
- _Solution_: Had to uninstall the Anaconda version of `libpq` using [this](https://github.com/abhi3700/My_Learning-Python#uninstallation). And then install using brew (which takes care of macOS M1 version).

```sh
$ brew install postgresql libpq
```

Then, go to your repo.

```sh
$ cargo clean
$ cargo build
$ cargo install diesel_cli --no-default-features --features postgres
```
