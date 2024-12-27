use super::story::{NewsSource, Story};
use super::{bandcamp, ft, hackernews, novara};
use crate::openai::summariser::summarise_article_text;
use regex::Regex;

async fn construct_hackernews() -> Story {
    match hackernews::process_top_stories().await {
        Ok(mut story) => {
            // Summarize article while the other Story structs are running.
            match summarise_article_text(&story).await {
                // Update content if summary is successful
                Ok(summary) => story.content = summary,
                // Use a default message on error
                Err(_) => story.content = "No summary of article available.".to_string(),
            }
            // Return story with either updated or default content
            story
        }
        Err(_) => Story::default_story(NewsSource::HackerNews),
    }
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

async fn construct_financialtimes() -> Story {
    ft::fetch_ft_daily()
        .await
        .unwrap_or_else(|_| Story::default_story(NewsSource::FinancialTimes))
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
    let (hackernews, novara, bandcamp, ft) = tokio::join!(
        construct_hackernews(),
        construct_novara(),
        construct_bandcamp(),
        construct_financialtimes()
    );

    
    vec![hackernews, novara, bandcamp, ft]
        .into_iter()
        .map(|mut story| {
            // Get rid of weird HTML artefacts in the email text.
            story.content = clean_html_content(&story.content);
            story
        })
        .collect::<Vec<Story>>()
}
