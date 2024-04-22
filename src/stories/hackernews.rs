use crate::parser::{fetch, misc};
use crate::stories::story::{NewsSource, Story};
use serde_json::Value;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StoryFetchError {
    #[error("Top stories JSON is not an array: {0}")]
    JsonNotArray(String),

    #[error("Failed to fetch any stories: {0}")]
    FetchFailure(String),
}

async fn fetch_top_stories() -> Result<Vec<Value>, StoryFetchError> {
    let top_stories_json =
        fetch::fetch_json("https://hacker-news.firebaseio.com/v0/topstories.json")
            .await
            .unwrap();
    match top_stories_json {
        Value::Array(top_stories) => Ok(top_stories),
        _ => Err(StoryFetchError::JsonNotArray(
            "Expected an array of story IDs".to_string(),
        )),
    }
}

async fn fetch_story_content(story_id: &Value) -> Result<Option<Story>, StoryFetchError> {
    let hn_top_story_url = format!(
        "https://hacker-news.firebaseio.com/v0/item/{}.json",
        story_id
    );
    let story_json = fetch::fetch_json(&hn_top_story_url).await.unwrap();

    match story_json["url"].as_str() {
        Some(url) => match fetch::get_html_body(url).await {
            Ok(html_body) if !html_body.trim().is_empty() => {
                let url = story_json["url"].as_str().unwrap_or_default().to_string();
                Ok(Some(Story {
                    url,
                    title: story_json["title"].to_string(),
                    news_source: NewsSource::HackerNews,
                    content: misc::parse_html_body(&html_body).to_string(),
                }))
            }
            _ => Ok(None),
        },
        None => Ok(None),
    }
}

pub async fn process_top_stories() -> Result<Story, StoryFetchError> {
    let top_stories = fetch_top_stories().await?;
    for story_id in top_stories.iter() {
        if let Some(story) = fetch_story_content(story_id).await? {
            if !story.content.is_empty() {
                return Ok(story);
            }
        }
    }
    Err(StoryFetchError::FetchFailure(
        "No valid stories with non-empty content found".to_string(),
    ))
}
