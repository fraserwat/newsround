use crate::stories::hackernews;
use crate::stories::novara;
use crate::stories::story::Story;
use std::error::Error;

mod parser;
mod stories;

async fn construct_hackernews() -> Result<Story, Box<dyn Error>> {
    let hn = hackernews::fetch_html_body_for_top_stories()
        .await
        .map_err(|_| "Nothing found!")?;
    Ok(hn)
}

async fn construct_novara() -> Result<Story, Box<dyn Error>> {
    let body = novara::fetch_latest_story().await?;
    Ok(body)
}

#[tokio::main]
async fn main() -> () {
    let hn_stripped_html: Story = construct_hackernews()
        .await
        .expect("Unable to fetch HackerNews story");
    let nm_stripped_html: Story = construct_novara()
        .await
        .expect("Unable to fetch Novara Media story");

    println!("HackerNews");
    println!("{:?}", hn_stripped_html);
    println!("Novara Media");
    println!("{:?}", nm_stripped_html);
}
