#[derive(Debug)]
pub enum NewsSource {
    HackerNews,
    Novara,
    Bandcamp,
    // FinancialTimes,
}

#[derive(Debug)]
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
            title: "".to_string(),
            url: "".to_string(),
            news_source: source,
            content: "".to_string(),
        }
    }
}
