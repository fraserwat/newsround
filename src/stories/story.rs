#[derive(Debug)]
pub enum NewsSource {
    HackerNews,
    Novara,
    Bandcamp,
    // FinancialTimes,
}

#[derive(Debug)]
// TODO: Check out https://rust-book.cs.brown.edu/ch06-01-defining-an-enum.html
// Move has named fields, like a struct does.
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
