use super::story::{NewsSource, Story};
use super::{bandcamp, hackernews, novara};
use crate::openai::summariser::summarise_article_text;

async fn construct_hackernews() -> Story {
    hackernews::process_top_stories()
        .await
        .unwrap_or_else(|_| Story::default_story(NewsSource::HackerNews))
}

async fn construct_novara() -> Story {
    novara::fetch_latest_story()
        .await
        .unwrap_or_else(|_| Story::default_story(NewsSource::Novara))
}

async fn construct_bandcamp() -> Story {
    bandcamp::fetch_bandcamp_daily()
        .await
        .unwrap_or_else(|_| Story::default_story(NewsSource::Bandcamp))
}

pub async fn generate_story_vector() -> Vec<Story> {
    let stories = vec![
        construct_hackernews().await,
        construct_novara().await,
        construct_bandcamp().await,
    ];

    let mut updated_stories = Vec::new();

    for mut story in stories {
        if let NewsSource::HackerNews = story.news_source {
            // println!("{:?}", story.content);
            match summarise_article_text(&story).await {
                Ok(summary) => story.content = summary,
                Err(_) => story.content = "No summary of article available.".to_string(),
            }
        }
        updated_stories.push(story);
    }

    // Return stories after updating HackerNews story with OpenAI summariser
    updated_stories
}
