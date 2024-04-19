use crate::openai::summariser::generate_email_subject;
use crate::stories::generate::generate_story_vector;
use chrono::{Datelike, Utc};

mod openai;
mod parser;
mod stories;

// TODO: Call email generator.

#[tokio::main]
async fn main() {
    let story_vector = generate_story_vector().await;
    let mut content_vector: Vec<String> = vec![];

    for story in story_vector {
        content_vector.push(story.content.clone());
    }
    let email_subject: String = generate_email_subject(content_vector)
        .await
        .unwrap_or_else(|_| {
            let now = Utc::now();
            format!(
                "News Roundup ({}-{:02}-{:02})",
                now.day(),
                now.month(),
                now.year()
            )
        });
}
