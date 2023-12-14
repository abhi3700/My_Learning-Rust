use chatgpt::prelude::*;
use chatgpt::types::CompletionResponse;

#[tokio::main]
async fn main() -> Result<()> {
    // Getting the API key here
    dotenv::dotenv().ok();
    dotenv::from_path("./.env").expect(".env file not found");

    // fetch the .env var
    let key = std::env::var("OPENAPI_KEY").expect("Key not found");

    // Creating a new ChatGPT client.
    // Note that it requires an API key, and uses
    // tokens from your OpenAI API account balance.
    let client = ChatGPT::new(key)?;

    // Sending a message and getting the completion
    let response: CompletionResponse = client
        .send_message("Describe in five words the Rust programming language.")
        .await?;

    println!("Response: {}", response.message().content);

    Ok(())
}
