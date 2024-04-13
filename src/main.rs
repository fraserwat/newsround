use crate::stories::story::Story;
use crate::stories::{bandcamp, hackernews, novara};
use std::error::Error;

mod parser;
mod stories;

async fn construct_hackernews() -> Result<Story, Box<dyn Error>> {
    let hn = hackernews::fetch_html_body_for_top_stories().await?;
    Ok(hn)
}

async fn construct_novara() -> Result<Story, Box<dyn Error>> {
    let nm = novara::fetch_latest_story().await?;
    Ok(nm)
}

async fn construct_bandcamp() -> Result<Story, Box<dyn Error>> {
    let bc = bandcamp::fetch_bandcamp_daily().await?;
    Ok(bc)
}

#[tokio::main]
async fn main() -> () {
    let hackernews_story: Story = construct_hackernews()
        .await
        .expect("Unable to fetch HackerNews story");
    let novara_story: Story = construct_novara()
        .await
        .expect("Unable to fetch Novara Media story");
    let bandcamp_daily: Story = construct_bandcamp()
        .await
        .expect("Unable to fetch Album of the Day");

    println!("HackerNews story: {}", hackernews_story.title);
    println!("Novara story: {}", novara_story.title);
    println!("Bandcamp Daily: {:?}", bandcamp_daily);
}
