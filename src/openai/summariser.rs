use super::api_response::OpenAIResponse;
use super::authenticate::authenticate_openai_api_key;
use crate::stories::story::Story;
use reqwest::Client;
use std::error::Error;

fn calculate_max_tokens(text: &str) -> usize {
    // Rough whitespace-based token counter to estimate what I need to set my max_token value to.
    let estimated_input_tokens = text.split_whitespace().count();
    let chatgpt4_max_tokens = 128_000;

    // Err on the side of caution +/- 20%
    (estimated_input_tokens as f64 * 1.2)
        .round()
        .min(chatgpt4_max_tokens as f64) as usize
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
        if let Some(first_choice) = parsed_response.choices.get(0) {
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
    let params = serde_json::json!({
        "model": "gpt-4-turbo",
        "max_tokens": calculate_max_tokens(&article.content),
        "messages": [
            {
                "role": "system",
                "content": "You do not refer to 'the text' in any way. You write in a way like you are writing the blurb of a news story on that website's front page. Be professional but not high-brow, and don't use lots of uneccessary adverbs."
            },
            {
                "role": "user",
                "content": format!("Summarise the text in between the '```' backticks succinctly in an dispassionate tone you might expect from a newsreader:\n\n```{}```\n\nKeep the summary to 50-100 words, within the context of the title of this article, {}. Make sure your description matches the tone of the piece.", article.content.replace("\"", "'"), article.title.replace("\"", "'")),
            }
        ]
    });

    open_ai_api_call(params).await
}

pub async fn generate_email_subject(titles: Vec<&str>) -> Result<String, Box<dyn Error>> {
    let params = serde_json::json!({
        "model": "gpt-4-turbo",
        "max_tokens": 100,
        "messages": [
            {
                "role": "system",
                "content": "Optimise for brevity. Only output three words which summarise the topics. Do not give any other output."
            },
            {
                "role": "user",
                "content": format!("Give a three word summary of the following titles\n\n\"{}\".\n\nIf there are more than three titles, ignore one at random. Remember, one line, three words. That's it!", format_titles_with_numbering(titles)),
            }
        ]
    });

    open_ai_api_call(params).await
}

fn format_titles_with_numbering(strings: Vec<&str>) -> String {
    // Some formatting of the titles ahead of the OpenAI prompt.
    strings
        .iter()
        .enumerate()
        .map(|(index, string)| format!("{}) {}", index + 1, string)) // Format string with numbering
        .collect::<Vec<_>>()
        .join("\n")
}
