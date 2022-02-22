pub const TWITTER_BASE_API: &str = "https://api.twitter.com/2/";

use serde::Deserialize;
use serde_json::Value;

pub const TWITTER_TWEET_SEARCH: &str = "tweets/search/recent/";
// pub const TWITTER_AUTH_TOKEN:&str="AAAAAAAAAAAAAAAAAAAAACuFZQEAAAAAwKKMQk7xK2rhCpTVctpBZqhzVxI%3DZEd7d5kCShK6kUtSJ7u0leE7XXv5i8v2Vgryc7jUihcE5yEQ0s";

// pub struct Error {
//     pub errors: Value,
// }

// pub struct Tweet {
//     pub data: Value,
// }
#[derive(Default, Deserialize, Debug)]
pub struct User {
    pub id: Option<String>,
    pub name: Option<String>,
    pub username: Option<String>,
    pub description: Option<String>,
    pub created_at: Option<String>,
}
#[derive(Default, Deserialize, Debug)]
pub struct Tweet {
    pub author_id: Option<String>,
    pub created_at: Option<String>,
    pub id: Option<String>,
    pub text: Option<String>,
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
    users(Vec<User>),
    DefaultVariant,
}

#[derive(Default, Deserialize, Debug)]
pub struct Tweets {
    pub data: Vec<Tweet>,
    pub includes: TwitterTypes,
    pub meta: Meta,
}

impl Default for TwitterTypes {
    fn default() -> Self {
        Self::DefaultVariant
    }
}
