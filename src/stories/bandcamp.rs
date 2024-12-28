use crate::parser::{bandcamp, fetch};
use crate::stories::story::{NewsSource, Story};
use std::error::Error;
use chrono::NaiveDate;

pub async fn fetch_bandcamp_daily() -> Result<Story, Box<dyn Error>> {
    // Want to take the latest article from one of these pages.
    let base_urls = [
        "https://daily.bandcamp.com/features",
        "https://daily.bandcamp.com/lists",
        // "https://daily.bandcamp.com/best-of-2025",
        "https://daily.bandcamp.com/album-of-the-day",
    ];

    // Vector for selecting latest article.
    let mut articles = Vec::new();

    for base_url in base_urls {
        let webpage = fetch::get_html_body(base_url).await?;
        let slug: String = base_url.to_string().replace("https://daily.bandcamp.com", "");

        if let Ok(Some((slug_url, pub_date_str))) = bandcamp::get_latest_entry_url(&webpage, &slug) {
            let article_url = format!("{base_url}{slug_url}");
            let pub_date = NaiveDate::parse_from_str(&pub_date_str, "%B %d, %Y")
                .expect("Failed to parse date");

            let daily_html = fetch::get_html_body(&article_url).await?;

            match bandcamp::get_latest_title_paragraph(&daily_html) {
                Ok((title, content)) => {
                    articles.push((pub_date, Story {
                        title,
                        url: article_url,
                        content,
                        news_source: NewsSource::Bandcamp,
                    }));
                },
                Err(e) => {
                    println!("{e:?}");
                }
            }
        }
    }
    // Return the most recent article, or an error if none were found
    match articles.into_iter().max_by_key(|(date, _)| *date) {
        Some((_, latest_story)) => Ok(latest_story),
        None => Err("No valid entries found. Sad!".into())
    }
}
