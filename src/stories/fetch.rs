use reqwest;
use serde_json::Value;
use std::error::Error;

pub async fn fetch_json(url: &str) -> Result<Value, Box<dyn Error>> {
    let response = reqwest::get(url).await?;
    let json: Value = response.json().await?;
    Ok(json)
}

pub async fn get_html_body(url: String) -> Result<String, reqwest::Error> {
    // Get HTML from url passed into function
    println!("{}", url);
    let response = reqwest::get(url).await?;
    // The "?" propagates any errors
    let response = response.error_for_status()?;

    // Return the text
    response.text().await
}
