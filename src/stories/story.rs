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
