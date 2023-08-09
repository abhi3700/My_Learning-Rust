# Redis

## Installation

```sh
$ cargo add redis
```

## Server

2 ways to spin up redis server:

### Local

Use local server via `$ redis-server` command. If not installed, then install it via `$ brew install redis` command.

### External

Use redis db instance from external service providers like heroku, render, AWS, etc.

Redis DB hosted on these platforms:

- [Render | All-in-One cloud service](https://dashboard.render.com/) (FREE plan | multiple DB w 30 MB each) comes with TLS support `Tested Ok ✅`
  - steps:
    1. create redis instance with a unique name.
    2. Now, under "Access Control" of the DB, press <kbd>Use my IP</kbd> button to allow my IP to access the DB.
    3. Copy the "External Redis URL" and use it in the code as environment variable.
- [Redis Enterprise Cloud](https://app.redislabs.com/) (FREE plan | 1 DB with 30MB ) doesn't have TLS support. `Tested Ok ✅`
- [Heroku](https://dashboard.heroku.com/) (No DBs available in FREE plan) ❌ (not preferred for testing)

## Redis CLI

After spinning up redis server, run the following command to open redis cli:

```sh
$ redis-cli
```

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
