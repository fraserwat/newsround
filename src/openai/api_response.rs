use serde::Deserialize;

// Structs to give API... "structure". Most of the stuff here isn't used, but have kept in comments
// in case I need it for something in the future, ease of use, etc.
// I'm sure there's a simpler way of doing this...

#[derive(Debug, Deserialize)]
pub struct OpenAIResponse {
    // id: String,
    // object: String,
    // created: i64,
    // model: String,
    pub choices: Vec<Choice>,
    // usage: Usage,
    // system_fingerprint: String,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    // role: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    // index: i64,
    pub message: Message,
    // logprobs: Option<serde_json::Value>,
    // finish_reason: Option<String>,
}

// #[derive(Debug, Deserialize)]
// struct Usage {
// prompt_tokens: i64,
// completion_tokens: i64,
// total_tokens: i64,
// }
