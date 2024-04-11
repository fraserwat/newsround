use serde_json::Value;
use std::error::Error;

use crate::stories::fetch;

pub async fn fetch_html_body_for_top_stories() -> Result<String, Box<dyn Error>> {
    let top_stories_json =
        fetch::fetch_json("https://hacker-news.firebaseio.com/v0/topstories.json").await?;

    if let Value::Array(top_stories) = top_stories_json {
        // Bias to the top story, but work our way down otherwise
        for story_id in top_stories.iter() {
            let hn_top_story_url = format!(
                "https://hacker-news.firebaseio.com/v0/item/{}.json",
                story_id
            );

            // Attempt to fetch the story JSON to get the URL
            match fetch::fetch_json(&hn_top_story_url).await {
                // If we successfully get a story
                Ok(story_json) => {
                    if let Some(url) = story_json["url"].as_str() {
                        // Attempt to fetch the HTML body
                        match fetch::get_html_body(url.to_string()).await {
                            // Success
                            Ok(html_body) => return Ok(html_body),
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
