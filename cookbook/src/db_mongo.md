---
title: Mongo DB
---

[examples](../../libs/databases/mongo/demo/examples/)

## Add to Cargo.toml

```toml
[dependencies]
mongodb = {version = "2.8.1", default-features = false, features = ["tokio-runtime"]}
tokio = { version = "1.21.2", features = ["full"] }
```

## Connect to Mongo DB

```rust
// connect to local DB.
let client = Client::with_uri_str("mongodb://localhost:27017").await?;
```

Attach a tokio executor to the `main` fn:

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // connect to local DB.
}
```

## Create

### Create collection in DB, if doesn't exist

```rust
async fn is_coll_in_db(client: &Client, db_name: &str, coll_name: &str) -> bool {
    let db = client.database(db_name);
    db.list_collection_names(None)
        .await
        .map(|collections| collections.contains(&coll_name.to_string()))
        .unwrap_or(false)
}


async fn create_collection(
    client: &Client,
    db_name: &str,
    coll_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let db = client.database(db_name);
    if !is_coll_in_db(client, db_name, coll_name).await {
        db.create_collection(coll_name, None).await?;
    }

    Ok(())
}
```

### Insert single document

```rust
async fn insert_document<T: Serialize>(
    client: &Client,
    db_name: &str,
    coll_name: &str,
    document: &T,
) -> Result<(), Box<dyn std::error::Error>> {
    let db = client.database(db_name);
    let coll: mongodb::Collection<T> = db.collection(coll_name);
    coll.insert_one(document, None).await?;

    Ok(())
}
```

And now use it in `main` function:

```rust
insert_document(&client, db_name, coll_name, &User { name: "John".to_string(), age: 28 })
    .await?;
```

## Read

### Read a single document with given filter

Suppose, you want to read John's info:

```rust
async fn get_document(
    client: &Client,
    db_name: &str,
    coll_name: &str,
    filter: Document,
) -> Result<(), Box<dyn std::error::Error>> {
    let db = client.database(db_name);
    let coll: mongodb::Collection<User> = db.collection(coll_name);
    let result = coll.find_one(filter, None).await?;

    match result {
        Some(doc) => println!("\t{:?}", doc),
        None => println!("\tNo document found"),
    }

    Ok(())
}
```

Now, use it in `main` function:

```rust
get_document(&client, db_name, coll_name, doc! {"name": "John"}).await?;
```

### Read all documents in a collection

M-1: Using `collect` into a vector of documents:

```rust
// connect to local DB.
// ...

let coll = client.database(db_name).collection(coll_name);
let docs = coll.find(None, None).await?.collect::<Vec<_>>();

// print the documents
for doc in documents {
    println!("{:#?}", doc?);
}
```

M-2: Using cursor:

```rust
// connect to local DB.
// ...

let coll = client.database(db_name).collection(coll_name);
let mut docs_cursor: Cursor<User> = coll.find(None, None).await?;

while let Some(doc) = docs_cursor.next().await {
    println!("{:#?}", doc?);
}
```

## Update

Update a document of given filter and then update it with another value. For instance, update John's age to 30:

```rust
async fn update_document(
    client: &Client,
    db_name: &str,
    coll_name: &str,
    filter: Document,
    update: Document,
) -> Result<(), Box<dyn std::error::Error>> {
    let db = client.database(db_name);

    let coll: mongodb::Collection<User> = db.collection(coll_name);
    coll.update_one(filter, update, None).await?;

    Ok(())
}
```

And now use it in `main` function:

```rust
update_document(&client, db_name, coll_name, doc! {"name": "John"}, doc! {"$set": {"age": 30}})
    .await?;
```

## Delete

Delete a document with given filter. For instance, delete John:

```rust
async fn delete_document(
    client: &Client,
    db_name: &str,
    coll_name: &str,
    filter: Document,
) -> Result<(), Box<dyn std::error::Error>> {
    let db = client.database(db_name);
    let coll: mongodb::Collection<User> = db.collection(coll_name);
    coll.delete_one(filter, None).await?;

    Ok(())
}
```

Now, use it in `main` function:

```rust
delete_document(&client, db_name, coll_name, doc! {"name": "John"}).await?;
```
