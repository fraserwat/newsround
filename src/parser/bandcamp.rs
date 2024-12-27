use scraper::{Html, Selector};
use std::error::Error;

pub fn get_latest_entry_url(html: &str, slug: &str) -> Result<Option<String>, Box<dyn Error>> {
    // Get html into a html parser
    let document = Html::parse_document(html);

    // Create a selector for the first child of the Album of the Day article list.
    let select_latest = Selector::parse("#p-daily-franchise > articles-list > *:nth-child(1) > a")?;
    // Create selector to extract the URL from the above element.
    let url_option = document
        .select(&select_latest).find_map(|element| element.value().attr("href"))
        .map(|u| u.to_string().replace(slug, ""));
    match url_option {
        Some(url) if !url.is_empty() => Ok(Some(url)),
        _ => Err("No entry found today!".into()),
    }
}

pub fn get_latest_title_paragraph(html: &str) -> Result<(String, String), Box<dyn Error>> {
    let document = Html::parse_document(html);
    // Grab first paragraph of article
    let title_selector = Selector::parse("article-title")?;
    let article_selector = Selector::parse("article > p:first-of-type")?;
    
    let title = document
    .select(&title_selector)
    .next()
    .map(|x| x.inner_html());

    let first_para = document
        .select(&article_selector)
        .next()
        .map(|x| x.inner_html());
    match (title, first_para) {
        (None, Some(_t)) => Err("Title not found in the HTML".into()),
        (Some(_t), None) => Err("First paragraph not found in the HTML".into()),
        (Some(t), Some(p)) if !t.is_empty() && !p.is_empty() => Ok((t, p)),
        _ => Err("Both title and first paragraph are missing or empty".into()),
    }
}
