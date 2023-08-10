# Distributed databases

## Overview

All the DBs are mostly

- NoSQL,
- distributed,
- horizontally and vertically scalable.

Available DBs are:

- [Apache Cassandra](https://cassandra.apache.org/_/index.html)
  - It has its own query language called CQL (Cassandra Query Language).
  - Came before Scylla with Java overhead using GC (Garbage Collection)
- [Scylla DB](https://www.scylladb.com/)
  - a better version of Cassandra because it is written primarily in rust.
  - It has its own query language called CQL (Cassandra Query Language).
  - [cloud service provider](https://cloud.scylladb.com/account/login) natively runs on GCP and AWS.
- Azure Cosmos DB
  - has SQL query support on NoSQL databases (MongoDB, Cassandra).
- AWS DynamoDB

## References

- [Getting started with Azure Cosmos DB using API for MongoDB and Rust](https://devblogs.microsoft.com/cosmosdb/mongodb-and-rust/)
- [Exploring Azure CosmosDB with Rust - Part 1](https://dev.to/mindflavor/exploring-azure-cosmosdb-with-rust-part-1-1ki7)
- [Exploring Azure CosmosDB with Rust - Part 2](https://dev.to/mindflavor/exploring-azure-cosmosdb-with-rust-part-2-32c0)
