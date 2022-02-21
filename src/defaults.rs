pub const TWITTER_BASE_API: &str = "https://api.twitter.com/2/";

use serde::Deserialize;
use serde_json::Value;
pub const TWITTER_TWEET_BY_ID_ENDPOINT: &str = "tweets/";
pub const TWITTER_TWEET_SEARCH: &str = "tweets/search/recent/";
// pub const TWITTER_AUTH_TOKEN:&str="AAAAAAAAAAAAAAAAAAAAACuFZQEAAAAAwKKMQk7xK2rhCpTVctpBZqhzVxI%3DZEd7d5kCShK6kUtSJ7u0leE7XXv5i8v2Vgryc7jUihcE5yEQ0s";

// pub struct Error {
//     pub errors: Value,
// }

// pub struct Tweet {
//     pub data: Value,
// }

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Response {
    Tweet { data: Value, meta: Value },
    Error { errors: Value },
}
