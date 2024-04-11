use parser::misc;
use std::error::Error;
use stories::hackernews;

mod parser;
mod stories;

async fn construct_hackernews() -> Result<String, Box<dyn Error>> {
    let body = hackernews::fetch_html_body_for_top_stories()
        .await
        .map_err(|_| "Nothing found!")?;
    Ok(misc::parse_html_body(&body))
}

#[tokio::main]
async fn main() -> () {
    let hn_stripped_html: String = construct_hackernews()
        .await
        .expect("Unable to fetch HackerNews story");
    println!("{}", hn_stripped_html);
}
