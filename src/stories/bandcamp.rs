use crate::parser::{bandcamp, fetch};
use crate::stories::story::{NewsSource, Story};
use std::error::Error;

pub async fn fetch_bandcamp_daily() -> Result<Story, Box<dyn Error>> {
    let base_urls = [
        "https://daily.bandcamp.com/features",
        "https://daily.bandcamp.com/lists",
        "https://daily.bandcamp.com/best-of-2025",
        "https://daily.bandcamp.com/album-of-the-day",
        ];

        for base_url in base_urls {
            let webpage = fetch::get_html_body(base_url).await?;
            let slug: String = base_url.to_string().replace("https://daily.bandcamp.com", "");

            if let Ok(Some(daily_url)) = bandcamp::get_latest_entry_url(&webpage, &slug) {
                let url = format!("{base_url}{daily_url}");
                let daily_html = fetch::get_html_body(&url).await?;

                match bandcamp::get_latest_title_paragraph(&daily_html) {
                    Ok((title, content)) => {
                        return Ok(Story {
                            title,
                            url,
                            content,
                            news_source: NewsSource::Bandcamp,
                        })
                    },
                    Err(e) => {
                        println!("{e:?}");
                    }
                }
            }
        }

        Err("No entry for today. Sad!".into())
    }