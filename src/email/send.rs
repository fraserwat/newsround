use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use std::env;
use std::error::Error;
use urlencoding::encode;

pub async fn send_email(subject: String, html_content: String) -> Result<(), Box<dyn Error>> {
    let domain = env::var("MAILGUN_DOMAIN").expect("MAILGUN_DOMAIN must be set");
    let to = env::var("EMAIL_TO").expect("EMAIL_TO must be set");
    let from = env::var("EMAIL_FROM").expect("EMAIL_FROM must be set");
    // TODO: Possible to encode API Key without auth issues?
    let api_key = format!(
        "Basic {}",
        env::var("MAILGUN_API_KEY").expect("MAILGUN_API_KEY must be set")
    );

    let url = format!(
        "https://api.mailgun.net/v3/{}/messages?from={}&to={}&subject={}&html={}",
        domain,
        encode(&from),
        encode(&to),
        encode(&subject),
        encode(&html_content)
    );

    let client = Client::new();
    let headers = {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(AUTHORIZATION, api_key.parse().unwrap());
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers
    };

    let response = client.post(url).headers(headers).send().await?;

    if response.status().is_success() {
        println!("Response body: {}", response.text().await?);
        Ok(())
    } else {
        let error_msg = format!("Failed to send email: {:?}", response.status());
        println!("Response body: {}", response.text().await?);
        return Err(error_msg.into());
    }
}
