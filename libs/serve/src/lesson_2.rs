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
    let new_todo = Todo {
        user_id: 1,
        id: None,
        title: "New Todo".to_string(),
        completed: false,
    };

    // `serde_json::Value` is a generic type which adapts to any JSON type
    let new_todo: serde_json::Value = reqwest::Client::new()
        .post("https://jsonplaceholder.typicode.com/todos")
        .json(&new_todo) // predefined JSON type above
        // .json(&serde_json::json!({      // random JSON object
        //     "user_id": 1,
        //     "title": "New Todo".to_string(),
        //     "completed": false,
        // }))
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", new_todo);

    Ok(())
}
