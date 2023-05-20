# Mongo DB

## Overview

MongoDB is a NoSQL (Document) database, which means it does not use a relational schema like SQL databases. Instead, MongoDB stores data in a flexible, JSON-like format called BSON (Binary JSON). Due to its schema-less nature, you don't need a traditional Object-Relational Mapping (ORM) tool to work with MongoDB as you would with a SQL database.

However, there are Object-Document Mapper (ODM) libraries available for different programming languages, which can be used as an equivalent to ORM in MongoDB. These libraries provide an abstraction layer and handle the conversion between the application's objects and the BSON documents stored in MongoDB.

For example, in JavaScript/Node.js, you have Mongoose, which is a popular ODM library for working with MongoDB. It provides a straightforward way to model your application data, validation, and query building.

Similarly, for other programming languages, there are libraries like:

- **Python**: MongoEngine and pymodm
- **Ruby**: Mongoid
- **Java**: Morphia
- **Rust**: mongodb-rust-driver (official driver) and wither

> Keep in mind that using an ODM is not mandatory when working with MongoDB. Depending on your use case and requirements, you may choose to use a lower-level driver or an ODM to interact with MongoDB more conveniently.

**ODM (Object-Document Mapping)**:

- Designed for document-based databases like MongoDB and Couchbase.
- Document databases store data as documents, which are usually in JSON or BSON (Binary JSON) format, allowing for a flexible and schema-less data structure.
- ODM maps objects in the application code to documents in the database, and it handles the conversion between the object-oriented programming model and the document model.
- ODM libraries often provide features like schema validation, document versioning, and query building.
- Examples of ODM libraries include Mongoose (JavaScript/Node.js), MongoEngine (Python), Mongoid (Ruby), and Morphia (Java).

## Libraries

- [diesel](https://crates.io/crates/diesel) (ORM based)
  - async ❌
- [diesel-async](https://crates.io/crates/diesel-async)
- [sea-orm](https://crates.io/crates/sea-orm)

## Reference

- [mongo crate](https://docs.rs/mongodb/latest/mongodb/index.html)
