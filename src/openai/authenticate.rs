use reqwest::Client;
use std::{env, error};

pub async fn authenticate_openai_api_key() -> Result<String, Box<dyn error::Error>> {
    // Get the OpenAI API key from the .dotenv file.
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");

    // Set up a reqwest API client for the OpenAI API.
    let client = Client::new();
    let response = client
        .get("https://api.openai.com/v1/engines")
        .header("Authorization", format!("Bearer {api_key}"))
        .send()
        .await?;

    // Check the API key response has a Success-ful status.
    if response.status().is_success() {
        Ok(api_key)
    } else {
        Err(format!("Authentication failed with status: {}", response.status()).into())
    }
}
