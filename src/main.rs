use crate::email::generate::render_newsletter;
use crate::openai::summariser::generate_email_subject;
use crate::stories::generate::generate_story_vector;
use chrono::{Datelike, Utc};

mod email;
mod openai;
mod parser;
mod stories;

#[tokio::main]
async fn main() {
    let mut story_vector = generate_story_vector().await;
    let mut content_vector: Vec<String> = vec![];

    match render_newsletter(&mut story_vector) {
        Ok(html) => println!("{}", html),
        Err(e) => eprintln!("Error rendering newsletter: {}", e),
    }

    for story in story_vector {
        content_vector.push(story.content.clone());
    }
    let email_subject: String = generate_email_subject(content_vector)
        .await
        .unwrap_or_else(|_| "News Roundup".to_string());

    let now = Utc::now();
    let email_subject = format!(
        "{:02}-{:02}-{}: {}",
        now.day(),
        now.month(),
        now.year(),
        email_subject
    );

    println!("{}", email_subject);
}
