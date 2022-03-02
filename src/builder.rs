use crate::client::TwitterClient;
use crate::defaults::TWITTER_BASE_API;
use std::collections::HashMap;
#[derive(Default)]
pub struct Builder {
    bearer_token: String,
    params: HashMap<String, String>,
}

impl Builder {
    pub fn set_bearer_token(mut self, bearer_token: String) -> Self {
        self.bearer_token = bearer_token;
        self
    }

    pub fn set_default_query_params(mut self, query_params: HashMap<String, String>) -> Self {
        self.params = query_params;
        self
    }

    pub fn build(self) -> Result<TwitterClient, String> {
        if self.bearer_token.len() == 0 {
            return Err("Bearer_Token is not defined!".to_string());
        }
        Ok(TwitterClient {
            bearer_token: self.bearer_token,
            params: self.params,
            base_api: TWITTER_BASE_API.to_string(),
        })
    }
}
