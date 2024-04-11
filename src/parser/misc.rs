use scraper::{Html, Selector};

pub fn parse_html_body(html: &str) -> String {
    // Get html into a html parser
    let document = Html::parse_document(html);

    // Create a selector for the content blocks to strip them out and isolate them.
    let selector = Selector::parse("h1, h2, p, strong, em, b, i").unwrap();

    // Execute the selector on the HTML document
    document
        .select(&selector)
        .map(|element| {
            if element.html().contains("<a") {
                " ".to_string()
            } else {
                element.html()
            }
        })
        .collect::<Vec<_>>()
        .join("/n")
}
