use crate::defaults::TweetH;
use http::header::AUTHORIZATION;
use reqwest::blocking::Client;
use reqwest::header;
use reqwest::Result;
pub struct TwitterExternal {
    client: Client,
}

impl TwitterExternal {
    pub fn new(authorization_token: &str) -> Self {
        let mut h = header::HeaderMap::new();
        h.insert(
            AUTHORIZATION,
            format!("Bearer {}", authorization_token).parse().unwrap(),
        );
        let client = Client::builder().default_headers(h).build().unwrap();
        Self { client }
    }
    pub fn get(&self, url: &str) -> Result<String> {
        let response: TweetH = self.client.get(url).send()?.json()?;
        Ok(response.data.text)
    }
}
