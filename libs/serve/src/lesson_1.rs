/*
    2 lessons learned:
    1. Converting JSON (response) to any Rust type (String, Vec<TODO>)
    2. Rust type to JSON (request)
*/
use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    #[serde(rename = "userId")]
    user_id: i32,
    id: Option<i32>,
    title: String,
    completed: bool,
}

#[tokio::main]
pub async fn run() -> Result<(), reqwest::Error> {
    // let todos: String = reqwest::Client::new()
    //     .get("https://jsonplaceholder.typicode.com/todos?userId=1")
    //     .send()
    //     .await?
    //     .text()      // converted to String type
    //     .await?;

    // println!("{}", todos);
    // println!("{}", type_of(todos));

    // ============
    let todos: Vec<Todo> = reqwest::Client::new()
        .get("https://jsonplaceholder.typicode.com/todos?userId=1")
        .send()
        .await?
        .json() // converted to JSON type
        .await?;

    println!("{:#?}", todos);
    // println!("{}", type_of(todos));
    Ok(())
}
