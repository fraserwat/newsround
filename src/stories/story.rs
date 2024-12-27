use serde::{Deserialize, Serialize};
use std::fmt;

// Serialize and Deserialize needed for Handlebars HTML templating.
#[derive(Debug, Serialize, Deserialize)]
pub struct Story {
    pub title: String,
    pub url: String,
    pub news_source: NewsSource,
    pub content: String,
}

impl Story {
    // Define a generic default Story creator function
    pub fn default_story(source: NewsSource) -> Story {
        Story {
            title: String::new(),
            url: String::new(),
            news_source: source,
            content: String::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum NewsSource {
    HackerNews,
    Novara,
    Bandcamp,
    FinancialTimes,
}

impl fmt::Display for NewsSource {
    // This simplifies the passing of news sources into the HTML constructor from the Handlebars crate.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                NewsSource::HackerNews => "Hacker News",
                NewsSource::Novara => "Novara Media",
                NewsSource::Bandcamp => "Bandcamp",
                NewsSource::FinancialTimes => "Financial Times",
            }
        )
    }
}
