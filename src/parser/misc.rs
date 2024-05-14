use scraper::{Html, Selector};
use std::error::Error;

pub fn parse_html_body(html: &str) -> Result<String, Box<dyn Error>> {
    // Get html into a html parser
    let document = Html::parse_document(html);

    // Create a selector for the content blocks to strip them out and isolate them.
    let selector = Selector::parse("p")?;

    // Execute the selector on the HTML document
    let content = document
        .select(&selector)
        .map(|x| x.inner_html())
        .collect::<Vec<_>>()
        .join("\n");

    Ok(content)
}
