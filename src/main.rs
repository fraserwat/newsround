use crate::openai::summariser::summarise_article_text;
use crate::stories::story::{NewsSource, Story};
use crate::stories::{bandcamp, hackernews, novara};

mod openai;
mod parser;
mod stories;

// Define a generic default Story creator function
fn default_story(source: NewsSource) -> Story {
    Story {
        title: "".to_string(),
        url: "".to_string(),
        news_source: source,
        content: "".to_string(),
    }
}

async fn construct_hackernews() -> Story {
    hackernews::process_top_stories()
        .await
        .unwrap_or_else(|_| default_story(NewsSource::HackerNews))
}

async fn construct_novara() -> Story {
    novara::fetch_latest_story()
        .await
        .unwrap_or_else(|_| default_story(NewsSource::Novara))
}

async fn construct_bandcamp() -> Story {
    bandcamp::fetch_bandcamp_daily()
        .await
        .unwrap_or_else(|_| default_story(NewsSource::Bandcamp))
}

#[tokio::main]
async fn main() {
    let stories = vec![
        construct_hackernews().await,
        construct_novara().await,
        construct_bandcamp().await,
    ];

    let mut updated_stories = Vec::new();

    for mut story in stories {
        if let NewsSource::HackerNews = story.news_source {
            match summarise_article_text(&story).await {
                Ok(summary) => story.content = summary,
                Err(_) => story.content = "No summary of article available.".to_string(),
            }
        }
        updated_stories.push(story);
    }

    // Print stories after potentially updating HackerNews stories
    for story in updated_stories {
        println!("\nStory: {:?}", story);
    }
}
