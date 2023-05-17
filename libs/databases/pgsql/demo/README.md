# PGSQL Demo

## Overview

- [ch-1](https://github.com/abhi3700/My_Learning-Rust/commit/fbe8bc1d5e3214b025c008a957fd6128ea4c9ed1)

## Setup

### Chapter-1

Added these files:

```sh
# files
src/main.rs
src/services.rs
```

1. `$ cargo add actix-web serde_json serde --features "derive"`

### Chapter-2

Added these files:

```sh
# files
src/schema.rs
src/db_models.rs
src/db_utils.rs
src/messages.rs
src/actors.rs

```

1. `$ cargo add diesel --features "diesel/postgres diesel/r2d2 diesel/chrono"`
2. `$ cargo add dotenv`
3. `$ diesel migration generate create_todo_table`

   > ensure `DATABASE_URL` is set in `.env` file

4. `$ diesel migration run` or `$ diesel print-schema > src/models/schema.rs`

   - in order to generate schema.rs in correct folder, run
   - add the SQL query for create, drop table in up.sql, down.sql respectively
   - `$ diesel migration run` -> generates `schema.rs` file (into the folder mentioned in `diesel.toml` file) with table macro `table!`.

   ```toml
   [print_schema]
    file = "src/models/schema.rs"
   ```

   > `$ diesel print-schema` is going to print the schema like this:

   ```rust
   // @generated automatically by Diesel CLI.

   diesel::table! {
       article (id) {
           id -> Int4,
           title -> Varchar,
           body -> Text,
           created_at -> Timestamp,
           updated_at -> Timestamp,
       }
   }
   ```

5. `$ diesel migration redo`
   - runs `down.sql` & then `up.sql` i.e. drop table & then create table.
6. `$ diesel_ext > src/models/db_models.rs`
   - generates the models for the table in `db_models.rs` file.
7. `$ cargo add actix`
   - implements the actor model for making the app concurrent with multiple actors.
8. `$ cargo add chrono`
9. `$ cargo add env_logger`

## Build

```sh
# debug mode
$ cargo build
# release mode
$ cargo build --release
```

## Run

```sh
$ cargo run
```
