use crate::parser::{bandcamp, fetch};
use crate::stories::story::{NewsSource, Story};
use std::error::Error;

pub async fn fetch_bandcamp_daily() -> Result<Story, Box<dyn Error>> {
    let base_url = "https://daily.bandcamp.com/album-of-the-day";
    let webpage = fetch::get_html_body(base_url).await?;

    let daily_url =
        bandcamp::get_latest_entry_url(&webpage)?.ok_or_else(|| "No entry for today. Sad!")?;

    // Add latest entry stub to url
    let url = format!("{}{}", base_url, daily_url);
    let daily_html = fetch::get_html_body(&url).await?;

    let (title, content) = bandcamp::get_latest_title_paragraph(&daily_html)?;

    Ok(Story {
        title,
        url,
        content,
        news_source: NewsSource::Bandcamp,
    })
}
