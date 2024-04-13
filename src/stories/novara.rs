use crate::stories::fetch;
use crate::stories::story::{NewsSource, Story};
use serde_json::Value;
use std::error::Error;

pub async fn fetch_latest_story() -> Result<Story, Box<dyn Error>> {
    let top_stories_json =
        fetch::fetch_json("http://novaramedia.com/wp-json/wp/v2/posts?per_page=5").await?;

    if let Value::Array(top_stories) = top_stories_json {
        for story in top_stories.iter() {
            // If it doesn't have a long content item then it is probably a video or podcast.
            let is_article = story["content"]["rendered"].to_string().len() > 1000;
            if is_article {
                return Ok(Story {
                    title: story["content"]["title"].to_string(),
                    news_source: NewsSource::Novara,
                    content: story["content"]["rendered"].to_string(),
                });
            } else {
                print!("No story #n");
                continue;
            };
        }
    };
    Err("No valid stories on Novara today!".into())
}
