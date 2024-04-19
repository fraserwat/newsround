use super::api_response::OpenAIResponse;
use super::authenticate::authenticate_openai_api_key;
use crate::stories::story::Story;
use reqwest::Client;
use std::error::Error;

fn calculate_max_tokens(text: &str) -> usize {
    // Rough whitespace-based token counter to estimate what I need to set my max_token value to.
    let estimated_input_tokens = text.split_whitespace().count();
    //
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

    // println!("{:?}", params);

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
        // println!("{:?}", response_text);
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
        // println!("Error {:?}: {:?}", status, error_message);
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
                "content": "You do not refer to 'the text', 'the author' or 'the article' in any way. In fact, avoid starting any sentences with 'The'. Write as if you were the author of the piece I am giving you, from their perspective (you shouldn't use the first person though, write as if this is an extract from the article). Don't use lots of uneccessary adverbs. Match the tone of the author."
            },
            {
                "role": "user",
                "content": format!("Summarise the article in between the '```' backticks succinctly:\n\n```{}```\n\nKeep the summary to 50-100 words, within the context of the title of this article, {}.", article.content.replace("\"", "'"), article.title.replace("\"", "'")),
            }
        ]
    });

    open_ai_api_call(params).await
}

pub async fn generate_email_subject(articles: Vec<String>) -> Result<String, Box<dyn Error>> {
    let params = serde_json::json!({
        "model": "gpt-4-turbo",
        "max_tokens": 100,
        "messages": [
            {
                "role": "system",
                "content": "Optimise for brevity. Only output three words which summarise the articles, one word per article. Do not give any other output. If a story sounds like an album review then try and name the sub-subgenre with one word (if it isn't, disregard this sentence). Be really specific, but this command can be overriden if the story pertains to any proper nouns (companies, countries, etc). Even in the proper noun case where there are more than one proper nouns in the content, pick the one people don't write about as much - your goal is to get me to click on the email subject line!"
            },
            {
                "role": "user",
                "content": format!("Give a three word summary of the following article summaries:\n\n\"{}\".\n\nIf there are more than three articles, ignore one at random. Remember, one line, three words for a email subject, each word should be capitalised like a title. That's it!", format_stories_with_numbering(articles)),
            }
        ]
    });

    open_ai_api_call(params).await
}

fn format_stories_with_numbering(strings: Vec<String>) -> String {
    // Some formatting of the article content bulletpoint summaries ahead of the OpenAI prompt.
    strings
        .iter()
        .enumerate()
        .map(|(index, string)| format!("{}) {}", index + 1, string)) // Format string with numbering
        .collect::<Vec<_>>()
        .join("\n")
}
