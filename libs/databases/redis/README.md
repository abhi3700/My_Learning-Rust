# Redis

## Installation

```sh
$ cargo add redis
```

## Server

Two ways to spin up redis server:

1. Use local server via `$ redis-server` command. If not installed, then install it via `$ brew install redis` command.
2. Use redis db instance from external service providers like heroku, [render](https://dashboard.render.com/), AWS, etc.

## Redis CLI

After spinning up redis server, run the following command to open redis cli:

```sh
$ redis-cli
```

## References

- redis driver:
  - [crates.io](https://crates.io/crates/redis)
  - [Github](https://github.com/redis-rs/redis-rs)
- [Redis website](https://redis.io/)
- [Redis commands](https://redis.io/commands/)
