use reqwest::Client;
use std::env;

pub async fn authenticate_openai_api_key() -> Result<String, String> {
    // Get the OpenAI API key from the .dotenv file.
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");

    // Set up a reqwest API client for the OpenAI API.
    let client = Client::new();
    let response = client
        .get("https://api.openai.com/v1/engines")
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await;

    // Check the API key response has a Success-ful status.
    match response {
        Ok(resp) if resp.status().is_success() => Ok(api_key),
        _ => Err(format!(
            "Authentication failed with status: {}",
            response.unwrap().status()
        )),
    }
}
