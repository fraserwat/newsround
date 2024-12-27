use scraper::{Html, Selector};
use std::error::Error;

pub fn get_top_story(html: &str) -> Result<(String, String, String), Box<dyn Error>> {
    // Get html into a html parser
    let document = Html::parse_document(html);

    // Create a selector for the top story on the FT UK homepage.
    let top_selector = Selector::parse(".o-teaser--top-story .o-teaser__heading > a").unwrap();
    let top_selector_lead =
        Selector::parse(".o-teaser__content > .o-teaser__standfirst a").unwrap();

    // Create selector to extract the URL from the above element.
    let url_option = document
        .select(&top_selector)
        .filter_map(|element| element.value().attr("href"))
        .next()
        .map(|u| "https://www.ft.com".to_owned() + u);
    // Get text from the title and subheader
    let title_option = document
        .select(&top_selector)
        .next()
        .map(|x| x.inner_html());
    let content_option = document
        .select(&top_selector_lead)
        .next()
        .map(|x| x.inner_html());

    // Combine results and return
    match (url_option, title_option, content_option) {
        (Some(url), Some(title), Some(content)) => Ok((url, title, content)),
        _ => Err("Failed to extract story components".into()),
    }
}
