# Mongo DB

A rust folder to learn about MongoDB.

Main folder is [here](https://github.com/abhi3700/My_Learning_Databases/blob/main/Mongo).

## Overview

- NoSQL DB
- Database(s)
  - Collection(s)
    - document(s) in BSON (more than just JSON ) format
      - field
        - value
- sample mongo db URI (not URL because it also can contain database_name after / slash):
  - `mongo://<host>:<port>` w/o username, password
  - `mongo://<username>:<password>@<host>:<port>` with username, password.

## [Installation](https://github.com/abhi3700/My_Learning_Databases/blob/main/Mongo/README.md#installation)

## Getting Started

Follow this [code example](./demo/) to get started.

## Server

### Local

refer the [installation](https://github.com/abhi3700/My_Learning_Databases/blob/main/Mongo/README.md#installation).

### [Cloud Service Providers](https://github.com/abhi3700/My_Learning_Databases/blob/main/Redis/README.md#cloud-service-providers)

## Notes

MongoDB is a NoSQL (Document) database, which means it does not use a relational schema like SQL databases. Instead, MongoDB stores data in a flexible, JSON-like format called BSON (Binary JSON). Due to its schema-less nature, you don't need a traditional Object-Relational Mapping (ORM) tool to work with MongoDB as you would with a SQL database.

However, there are Object-Document Mapper (ODM) libraries available for different programming languages, which can be used as an equivalent to ORM in MongoDB. These libraries provide an abstraction layer and handle the conversion between the application's objects and the BSON documents stored in MongoDB.

For example, in JavaScript/Node.js, you have Mongoose, which is a popular ODM library for working with MongoDB. It provides a straightforward way to model your application data, validation, and query building.

Similarly, for other programming languages, there are libraries like:

- **Python**: MongoEngine and pymodm
- **Ruby**: Mongoid
- **Java**: Morphia
- **Rust**: mongodb-rust-driver - [crate](https://crates.io/crates/mongodb/), [documentation](https://docs.rs/mongodb/latest/mongodb/index.html) supporting official mongo db releases.

> Keep in mind that using an ODM is not mandatory when working with MongoDB. Depending on your use case and requirements, you may choose to use a lower-level driver or an ODM to interact with MongoDB more conveniently.

**ODM (Object-Document Mapping)**:

- Designed for document-based databases like MongoDB and Couchbase.
- Document databases store data as documents, which are usually in JSON or BSON (Binary JSON) format, allowing for a flexible and schema-less data structure.
- ODM maps objects in the application code to documents in the database, and it handles the conversion between the object-oriented programming model and the document model.
- ODM libraries often provide features like schema validation, document versioning, and query building.
- Examples of ODM libraries include Mongoose (JavaScript/Node.js), MongoEngine (Python), Mongoid (Ruby), and Morphia (Java).

---

**The best parts of MongoDB in comparison to SQL counterparts are**:

- it is **schema-less for document** (analogous to table in SQL DB). This means that you don't need to define a schema for your data before you start using it. You can store any data in any format in a MongoDB collection, and you can change the schema of your data at any time without having to update the database schema.
- **iterate over the documents** in a collection using a cursor. A cursor is a pointer to the result set of a query. By default, the cursor returns all documents in the result set. However, you can use the `limit()` method to specify the maximum number of documents to return, and the `skip()` method to specify the number of documents to skip in the result set.
  > Hence, this doesn't require `join` operation unlike SQL DB tables.

## Architecture

Refer [this](https://github.com/abhi3700/My_Learning_Databases/blob/main/all.drawio).

## Coding

- The driver supports both asynchronous and synchronous runtimes.
- You can connect to your MongoDB deployment by providing a **connection URI**, also called a _connection string_, which instructs the driver on how to connect to a MongoDB deployment and how to behave while connected.
- The connection string includes:
  - the hostname or IP address and port of your deployment, 
  - the authentication mechanism, 
  - user credentials when applicable, and 
  - connection options.

## Libraries

- [diesel](https://crates.io/crates/diesel) (ORM based)
  - async ❌
- [diesel-async](https://crates.io/crates/diesel-async)
- [sea-orm](https://crates.io/crates/sea-orm)

## Advanced

For async-std support, refer this [code](https://github.com/mongodb-developer/rust-quickstart-code/tree/async-std).

---

For tokio support, refer this [code](https://github.com/mongodb-developer/rust-quickstart-code/tree/tokio).

---

For sync support (without an async support), refer this [code](https://github.com/mongodb-developer/rust-quickstart-code/tree/sync).

## Reference

- mongodb-rust-driver [Official]
  - [crate](https://crates.io/crates/mongodb/),
- [MongoDB Rust Driver](https://www.mongodb.com/docs/drivers/rust/current/)
  - [Usage Examples](https://www.mongodb.com/docs/drivers/rust/current/usage-examples/)
- [Get Started with Rust and MongoDB](https://www.mongodb.com/developer/languages/rust/rust-mongodb-crud-tutorial/#starting-your-project) 🧑🏻‍💻
- [Getting Started with Aggregation Pipelines in Rust](https://www.mongodb.com/developer/languages/rust/rust-quickstart-aggregation/)
- [Code snippets as Quick Reference](https://www.mongodb.com/docs/drivers/rust/current/quick-reference/)