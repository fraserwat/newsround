use crate::parser::{bandcamp, fetch};
use crate::stories::story::{NewsSource, Story};
use std::error::Error;

pub async fn fetch_bandcamp_daily() -> Result<Story, Box<dyn Error>> {
    let mut url: String = "https://daily.bandcamp.com/album-of-the-day".to_string();
    // let err_msg = "No entry for today. Sad!".into();
    match fetch::get_html_body(&url).await {
        Ok(webpage) => match bandcamp::get_latest_entry_url(&webpage) {
            Ok(daily_url) => {
                // Add latest entry stub to url
                url.push_str(daily_url.as_ref().unwrap());
                let daily_html = fetch::get_html_body(&url).await?;
                match bandcamp::get_latest_title_paragraph(&daily_html) {
                    Ok((title, paragraph)) => {
                        return Ok(Story {
                            title,
                            url,
                            news_source: NewsSource::Bandcamp,
                            content: paragraph,
                        });
                    }
                    Err(e) => {
                        return Err(e.into());
                    }
                };
            }
            Err(e) => Err(e.into()),
        },
        Err(e) => Err(e.into()),
    }
}
