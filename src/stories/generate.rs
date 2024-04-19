use super::story::{NewsSource, Story};
use super::{bandcamp, hackernews, novara};
use crate::openai::summariser::summarise_article_text;
use regex::Regex;

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

fn clean_html_content(input: &str) -> String {
    // Replace common HTML entities.
    let mut result = input.to_string();
    result = result.replace("[&hellip;]", "...");
    result = result.replace("&hellip;", "...");
    // Replace en / em dash.
    result = result.replace("&#8211;", "—");
    result = result.replace("&#8212;", "—");
    // Get rid of hardcoded returns.
    result = result.replace("\\n", "");

    // Remove HTML tags
    let tags_regex = Regex::new(r"(<!--.*?-->|<[^>]*>)").unwrap();
    result = tags_regex.replace_all(&result, "").to_string();

    // Truncate at the last full stop before an ellipsis if present.
    if let Some(ellipsis_pos) = result.find("...") {
        if let Some(last_period) = result[..ellipsis_pos].rfind('.') {
            result = result[..last_period + 1].to_string();
        }
    }

    result
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
            // Handling HackerNews stories
            match summarise_article_text(&story).await {
                Ok(summary) => story.content = summary,
                Err(_) => story.content = "No summary of article available.".to_string(),
            }
        } else {
            // Clean HTML content for all other news sources
            story.content = clean_html_content(&story.content);
        }
        updated_stories.push(story);
    }

    // Return stories after updating HackerNews story with OpenAI summariser
    updated_stories
}
