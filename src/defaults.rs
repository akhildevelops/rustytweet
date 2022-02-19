pub const TWITTER_BASE_API: &str = "https://api.twitter.com/2/";

use serde::Deserialize;
pub const TWITTER_TWEET_BY_ID_ENDPOINT: &str = "tweets/";
pub const TWITTER_AUTH_TOKEN:&str="AAAAAAAAAAAAAAAAAAAAACuFZQEAAAAAwKKMQk7xK2rhCpTVctpBZqhzVxI%3DZEd7d5kCShK6kUtSJ7u0leE7XXv5i8v2Vgryc7jUihcE5yEQ0s";

#[derive(Deserialize)]
pub struct Tweet {
    pub id: String,
    pub text: String,
}

#[derive(Deserialize)]
pub struct TweetH {
    pub data: Tweet,
}
