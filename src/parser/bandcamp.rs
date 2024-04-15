use scraper::{Html, Selector};
use std::error::Error;

pub fn get_latest_entry_url(html: &str) -> Result<Option<String>, Box<dyn Error>> {
    // TODO: Option types are only intended to wrap structs. What should I use instead?
    // Get html into a html parser
    let document = Html::parse_document(html);

    // Create a selector for the first child of the Album of the Day article list.
    let select_latest =
        Selector::parse("#p-daily-franchise > articles-list > *:nth-child(1) > a").unwrap();
    // Create selector to extract the URL from the above element.
    let url_option = document
        .select(&select_latest)
        .filter_map(|element| element.value().attr("href"))
        .next()
        .map(|u| u.to_string().replace("/album-of-the-day", ""));

    match url_option {
        Some(url) if !url.is_empty() => Ok(Some(url)),
        _ => Err("No entry found today!".into()),
    }
}

pub fn get_latest_title_paragraph(html: &str) -> Result<(String, String), Box<dyn Error>> {
    let document = Html::parse_document(html);
    // Grab first paragraph of article
    let title_selector = Selector::parse(".aotd .title-wrapper a").unwrap();
    let article_selector = Selector::parse(".aotd p").unwrap();

    let title = document
        .select(&title_selector)
        .next()
        .map(|x| x.inner_html());

    let first_para = document
        .select(&article_selector)
        .next()
        .map(|x| x.inner_html());

    match (title, first_para) {
        (None, _) => Err("Title not found in the HTML".into()),
        (_, None) => Err("First paragraph not found in the HTML".into()),
        (Some(t), Some(p)) if !t.is_empty() && !p.is_empty() => Ok((t, p)),
        _ => Err("Both title and first paragraph are missing or empty".into()),
    }
}
