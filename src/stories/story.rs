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
