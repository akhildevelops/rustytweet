pub const TWITTER_BASE_API: &str = "https://api.twitter.com/2/";

use serde::Deserialize;
use serde_json::Value;
pub const TWITTER_TWEET_SEARCH: &str = "tweets/search/recent/";

#[derive(Default, Deserialize, Debug)]
pub struct User {
    pub id: Option<String>,
    pub name: Option<String>,
    pub username: Option<String>,
    pub description: Option<String>,
    pub created_at: Option<String>,
    pub conversation_id: Option<String>,
    pub in_reply_to_user_id: Option<String>,
}

#[derive(Default, Deserialize, Debug)]
pub struct Tweet {
    pub author_id: Option<String>,
    pub created_at: Option<String>,
    pub id: Option<String>,
    pub text: Option<String>,
    pub attachments: Option<Attachments>,
    pub context_annotations: Option<Value>,
    pub conversation_id: Option<String>,
    pub entities: Option<Value>,
    pub geo: Option<Value>,
    pub in_reply_to_user_id: Option<String>,
    pub lang: Option<String>,
    pub referenced_tweets: Option<Value>,
    pub source: Option<String>,
}
// https://developer.twitter.com/en/docs/twitter-api/data-dictionary/object-model/tweet

#[derive(Deserialize, Debug)]
pub enum Attachments {
    #[serde(rename = "poll_ids")]
    POLLIDS(Vec<String>),
    #[serde(rename = "media_keys")]
    MEDIAKEYS(Vec<String>),
}

#[derive(Default, Deserialize, Debug)]
pub struct Meta {
    pub newest_id: Option<String>,
    pub next_token: Option<String>,
    pub oldest_id: Option<String>,
    pub result_count: Option<usize>,
}
#[derive(Deserialize, Debug)]
pub enum TwitterTypes {
    Tweet(Tweet),
    #[serde(rename = "users")]
    Users(Vec<User>),
    DefaultVariant,
}

#[derive(Default, Deserialize, Debug)]
pub struct Tweets {
    pub data: Vec<Tweet>,
    pub includes: TwitterTypes,
    pub meta: Meta,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum TweetsError {
    Tweet(Tweets),
    Error(Error),
    DefaultVariant,
}

#[derive(Deserialize, Debug)]
pub struct ErrorContent {
    pub message: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Error {
    pub title: String,
    pub detail: String,
    #[serde(rename = "type")]
    pub error_type: String,
    pub errors: Vec<ErrorContent>,
}

impl Default for TweetsError {
    fn default() -> Self {
        Self::DefaultVariant
    }
}

impl Default for TwitterTypes {
    fn default() -> Self {
        Self::DefaultVariant
    }
}
