use crate::defaults::{TWITTER_AUTH_TOKEN, TWITTER_BASE_API};
use crate::endpoints::{Twitter, TwitterEndPoints};
use crate::external::network::TwitterExternal;

pub fn search_tweets<'a>(tweet_id: &str) -> String {
    let tweet = Twitter::new(TWITTER_BASE_API).tweet_id(tweet_id);
    TwitterExternal::new(TWITTER_AUTH_TOKEN)
        .get(&tweet)
        .unwrap()
}
