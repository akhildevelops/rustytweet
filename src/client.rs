use super::expansion::Expansion;
use super::external::network::RequestInterface;
use crate::builder::Builder;
use crate::defaults;
use reqwest::Error;
use serde::de::DeserializeOwned;
use std::marker::PhantomData;

use std::collections::HashMap;
#[derive(Default)]
pub struct TwitterHandle<T> {
    base_api: String,
    expansions: Option<String>,
    fields: Option<HashMap<String, String>>, // Check on how to references internal sturct values. Here fields key shud reference expansion of sort.
    bearer_token: String,
    query: String,
    query_params: HashMap<String, String>,
    phantom: PhantomData<T>,
}

impl<T> TwitterHandle<T>
where
    T: DeserializeOwned + Default,
{
    pub fn new(
        query: String,
        query_params: HashMap<String, String>,
        base_api: String,
        bearer_token: String,
    ) -> Self {
        let mut th = Self::default();
        th.base_api = base_api;
        th.bearer_token = bearer_token;
        th.query = query;
        th.query_params = query_params;
        th
    }

    pub fn expansion(mut self, expansions: &[Expansion]) -> Self {
        let mut expansions_names = vec![];
        let mut exapansion_fields = HashMap::new();
        for expansion in expansions {
            let name = expansion.get_identifier();
            expansions_names.push(name.clone());
            exapansion_fields.insert(
                expansion.get_fields_identifier(),
                expansion.get_fields().join(","),
            );
        }
        match expansions_names.len() {
            0 => {
                self.expansions = None;
                self.fields = None
            }
            _ => {
                let names = expansions_names
                    .into_iter()
                    .filter(|x| x != &"")
                    .collect::<Vec<_>>()
                    .join(",");
                self.expansions = Some(names.trim().to_string());
                self.fields = Some(exapansion_fields);
            }
        }
        self
    }

    pub fn send(&self) -> Result<T, Error> {
        let response = self.get()?;

        Ok(response.json()?)
    }
}

impl<T> RequestInterface for TwitterHandle<T> {
    fn url(&self) -> String {
        self.base_api.clone()
    }

    fn bearer_auth(&self) -> String {
        self.bearer_token.clone()
    }

    fn query_params(&self) -> HashMap<String, String> {
        let mut hm = HashMap::new();
        hm.insert("query".to_string(), self.query.clone());
        if let Some(x) = &self.expansions {
            hm.insert("expansions".to_string(), x.clone());
        }
        if let Some(x) = &self.fields {
            for (k, v) in x.iter() {
                hm.insert(k.clone(), v.clone());
            }
        }
        hm
    }
}
pub struct TwitterClient {
    pub bearer_token: String,
    pub params: HashMap<String, String>,
    pub base_api: String,
}

impl TwitterClient {
    pub fn builder() -> Builder {
        Builder::default()
    }
    pub fn search_recent_tweets(&self, query: &str) -> TwitterHandle<defaults::Tweets> {
        let base_api = format!("{}{}", self.base_api, defaults::TWITTER_TWEET_SEARCH);
        TwitterHandle::new(
            query.to_string(),
            self.params.clone(),
            base_api,
            self.bearer_token.clone(),
        )
    }
}
