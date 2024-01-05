# Redis

A rust folder to learn about MongoDB.

Main folder is [here](https://github.com/abhi3700/My_Learning_Databases/blob/main/Redis).

## Overview

- key-value in-memory database
- supports multiple data types like string, list, set, sorted set, hash, etc.
- sample redis URL:
  - `redis://<username>:<password>@<host>:<port>` without TLS support
  - `rediss://<username>:<password>@<host>:<port>` with TLS support

## [Installation](https://github.com/abhi3700/My_Learning_Databases/blob/main/Redis/README.md#installation)

## Usage

```sh
cargo add redis
```

## Getting Started

Follow this [code example](./demo/) to get started.

## Server

2 ways to spin up redis server:

### Local

Use local server via `$ redis-server` command. If not installed, then install it via `$ brew install redis` command.

After spinning up redis server, run the following command to open redis cli:

```sh
redis-cli
```

### [Cloud Service Providers](https://github.com/abhi3700/My_Learning_Databases/blob/main/Redis/README.md#cloud-service-providers)

## Architecture

Refer [this](https://github.com/abhi3700/My_Learning_Databases/blob/main/all.drawio).

## Advanced

For custom data types like `struct`, `enum`, etc. to be stored in redis db, we need to implement `serde` traits for them, then use these dependencies:

```toml
[dependencies]
redis-macros = "0.1.0"
redis = { version = "0.22.2" }
serde = { version = "1.0.152", features = ["derive"] }
serde_json = { version = "1.0.91" }
```

---

For client with **Async support**, then add the corresponding feature from [here](https://github.com/redis-rs/redis-rs#async-support)

```toml
# if you use tokio
redis = { version = "0.23.1", features = ["tokio-comp"] }

# if you use async-std
redis = { version = "0.23.1", features = ["async-std-comp"] }
```

---

If your redis URL requires **TLS support**, then add the corresponding feature from [here](https://github.com/redis-rs/redis-rs#tls-support):

```toml
redis = { version = "0.23.1", features = ["tls-native-tls"] }

# if you use tokio
redis = { version = "0.23.1", features = ["tokio-native-tls-comp"] }

# if you use async-std
redis = { version = "0.23.1", features = ["async-std-native-tls-comp"] }
```

---

If you need cluster support i.e. distributed redis db implementation like sharding for _scalability_ (breaking the DB into clusters with mapping by keys) and _availability_ (if 1 node fails, data can be retrieved from other nodes), then add the corresponding feature from [here](https://github.com/redis-rs/redis-rs#cluster-support).

Async Redis Cluster support can be enabled by enabling the cluster-async feature, along with your preferred async runtime, e.g.:

```toml
redis = { version = "0.23.1", features = [ "cluster-async", "tokio-std-comp" ] }
```

---

In order to get access to JSON commands like `json_set()`, add the corresponding feature from [here](https://github.com/redis-rs/redis-rs#json-support)

## References

- redis driver:
  - [crates.io](https://crates.io/crates/redis)
  - [Github](https://github.com/redis-rs/redis-rs)
- [Redis website](https://redis.io/)
- [Redis commands](https://redis.io/commands/)
