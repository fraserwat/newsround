use crate::parser::{fetch, ft};
use crate::stories::story::{NewsSource, Story};
use std::error::Error;

pub async fn fetch_ft_daily() -> Result<Story, Box<dyn Error>> {
    let base_url = "https://www.ft.com/world-uk";
    let webpage = fetch::get_html_body(base_url).await?;

    let (url, title, content) = ft::get_top_story(&webpage)?;

    Ok(Story {
        title,
        url,
        content,
        news_source: NewsSource::FinancialTimes,
    })
}
