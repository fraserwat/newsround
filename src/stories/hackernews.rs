use crate::parser::{fetch, misc};
use crate::stories::story::{NewsSource, Story};
use serde_json::Value;
use std::error::Error;

pub async fn fetch_html_body_for_top_stories() -> Result<Story, Box<dyn Error>> {
    let top_stories_json =
        fetch::fetch_json("https://hacker-news.firebaseio.com/v0/topstories.json").await?;

    if let Value::Array(top_stories) = top_stories_json {
        // Bias to the top story, but work our way down otherwise
        for story_id in top_stories.iter() {
            let hn_top_story_url = format!(
                "https://hacker-news.firebaseio.com/v0/item/{}.json",
                story_id
            );

            // TODO: Move this into its own function
            // Attempt to fetch the story JSON to get the URL
            match fetch::fetch_json(&hn_top_story_url).await {
                // If we successfully get a story
                Ok(story_json) => {
                    if let Some(url) = story_json["url"].as_str() {
                        // Attempt to fetch the HTML body
                        match fetch::get_html_body(url).await {
                            // Success
                            Ok(html_body) => {
                                return Ok(Story {
                                    // TODO: Look at "struct update syntax" to clean this up.
                                    title: story_json["title"].to_string(),
                                    url: hn_top_story_url,
                                    news_source: NewsSource::HackerNews,
                                    content: misc::parse_html_body(&html_body).to_string(),
                                });
                            }
                            // Try the next story upon error
                            Err(_) => continue,
                        }
                    }
                }
                // Try the next story if fetching story JSON fails
                Err(_) => continue,
            }
        }
        Err("Failed to fetch any stories".into()) // If all attempts fail
    } else {
        Err("Top stories JSON is not an array".into()) // If the initial fetch does not return an array
    }
}
