use scraper::{Html, Selector};

pub fn parse_html_body(html: &str) -> String {
    // Get html into a html parser
    let document = Html::parse_document(html);

    // Create a selector for the content blocks to strip them out and isolate them.
    let selector = Selector::parse("p").unwrap();

    // Execute the selector on the HTML document
    document
        .select(&selector)
        .map(|x| x.inner_html())
        .collect::<Vec<_>>()
        .join("\n")
}
