use std::collections::HashMap;

use reqwest::blocking::Client;
use reqwest::blocking::Response;
use reqwest::Result;
// pub struct TwitterExternal {
//     client: Client,
// }

// impl TwitterExternal {
//     pub fn new(authorization_token: &str) -> Self {
//         let mut h = header::HeaderMap::new();
//         h.insert(
//             AUTHORIZATION,
//             format!("Bearer {}", authorization_token).parse().unwrap(),
//         );
//         let client = Client::builder().default_headers(h).build().unwrap();
//         Self { client }
//     }
//     pub fn get(&self, url: &str) -> Result<String> {
//         let response: TweetH = self.client.get(url).send()?.json()?;
//         Ok(response.data.text)
//     }
// }

pub trait RequestInterface {
    // fn headers(&self) -> HashMap<String, String>;
    fn url(&self) -> String;
    fn bearer_auth(&self) -> String;
    fn query_params(&self) -> HashMap<String, String>;
    fn get(&self) -> Result<Response> {
        // let headers = self.headers(); // add header map
        let url = self.url();

        let token = self.bearer_auth();
        let query_params = self.query_params();
        let client = Client::new();
        let req_builder = client.get(url).bearer_auth(token).query(&query_params);
        req_builder.send()
    }
}
