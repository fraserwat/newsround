use super::api_response::OpenAIResponse;
use super::authenticate::authenticate_openai_api_key;
use crate::stories::story::Story;
use reqwest::Client;
use std::error::Error;

fn calculate_max_tokens(text: &str) -> usize {
    // Rough whitespace-based token counter to estimate what I need to set my max_token value to.
    let estimated_input_tokens = text.split_whitespace().count();
    // let chatgpt4_max_tokens = 128_000;
    let chatgpt4_turbo_max_tokens = 4096;
    // Err on the side of caution +/- 20%
    (estimated_input_tokens as f64 * 1.2)
        .round()
        .min(chatgpt4_turbo_max_tokens as f64) as usize
}

async fn open_ai_api_call(params: serde_json::Value) -> Result<String, Box<dyn Error>> {
    // Get OpenAI API Key
    let api_key = authenticate_openai_api_key().await?;
    // Create API Client
    let client = Client::new();
    // Get response from API
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&params)
        .send()
        .await?;
    if response.status().is_success() {
        let response_text = response.text().await?;
        let parsed_response: OpenAIResponse = serde_json::from_str(&response_text)?;
        // Check if there is at least one choice and message
        if let Some(first_choice) = parsed_response.choices.first() {
            // Return the message content
            Ok(first_choice.message.content.clone())
        } else {
            Err("No message available in OpenAPI Response".into())
        }
    } else {
        let status = response.status();
        let error_message = response
            .text()
            .await
            .unwrap_or_else(|_| "Failed to get error message".to_string());
        Err(format!("Error {}: {}", status, error_message).into())
    }
}

pub async fn summarise_article_text(article: &Story) -> Result<String, Box<dyn Error>> {
    // For readability, splitting out the messages and object definition.
    let system_content: String = r"You do not refer to 'the text', 'the author' or 'the article' in any way.\ 
    Avoid starting any sentences with 'The'. Write as if you were the author of the piece I am giving you,\ 
    from their perspective (instead of writing in first person, write as if its an extract from the article).\ 
    Don't use uneccessary adverbs. Match the tone of the author, but keep it user-friendly.\ 
    Do not include URLs, as we already have a link to the article.".to_string();
    let user_content: String = format!(r"Summarise the article in between the '```' backticks in ~50 words:\n
    \n```{}```\n\nKeep the summary to roughly 50 words, within the context of the title of this article, {}.\ 
    Remember, 50 word limit, user-friendly, only the important info. So `versatile solution` would just be `solution`,\ 
    `it utilizes Docker for simple installation` would be written instead as `it uses Docker for installation`.\ 
    ", article.content.replace("\"", "'"), article.title.replace("\"", "'")).to_string();
    
    // Obj definition
    let params = serde_json::json!({
        "model": "gpt-4-turbo",
        "max_tokens": calculate_max_tokens(&article.content),
        "messages": [
            {
                "role": "system",
                "content": system_content
            },
            {
                "role": "user",
                "content": user_content,
            }
        ]
    });

    open_ai_api_call(params).await
}
