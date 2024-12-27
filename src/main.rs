use crate::email::generate::render_newsletter;
use crate::email::send::send_email;
use crate::stories::generate::generate_story_vector;
use chrono::{Datelike, Utc};
use std::env;
use std::path::Path;
use std::time::Instant;

mod email;
mod openai;
mod parser;
mod stories;

async fn run() -> (String, String) {
    let mut story_vector = generate_story_vector().await;
    let mut content_vector: Vec<String> = vec![];
    let now = Utc::now();
    let today = format!("{:02}-{:02}-{}", now.day(), now.month(), now.year());

    let html = match render_newsletter(&mut story_vector) {
        Ok(html) => {
            for story in story_vector {
                content_vector.push(story.content.clone());
            }
            html
        }
        Err(e) => {
            format!("Error rendering newsletter: {}", e)
        }
    };

    let formatted_subject = format!("News Roundup: {}", today);

    (html, formatted_subject)
}

#[tokio::main]
async fn main() {
    // Load Environment Variables
    let env_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(".env");
    dotenv::from_path(env_path.as_path()).ok();
    println!("Environment setup with .env from: {:?}", env_path);

    let now = Instant::now();

    {
        let (html, subject) = run().await;

        match send_email(subject.clone(), html).await {
            Ok(_) => println!("Email Successfully Sent: {}", subject),
            Err(e) => println!("Error: {}", e),
        };
    }

    let elapsed = now.elapsed().as_secs();

    // Last 5 Pre-Parallelism Runtimes (sec): [11, 14, 12, 13, 14] (avg = 12.8 sec)
    // Last 5 Post-Parallelism Runtimes (sec): [11, 8, 8, 9, 10] (avg = 9.2 sec)
    println!("Runtime: {:.2?}s", elapsed);
}
