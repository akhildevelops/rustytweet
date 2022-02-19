use crate::defaults::TWITTER_TWEET_BY_ID_ENDPOINT;

pub trait TwitterEndPoints {
    fn tweet_id(&self, id: &str) -> String;
}
pub struct Twitter<'a> {
    base_end_point: &'a str,
}

impl<'a> Twitter<'a> {
    pub fn new(base_end_point: &'a str) -> Self {
        Self { base_end_point }
    }
}

impl<'a> TwitterEndPoints for Twitter<'a> {
    fn tweet_id(&self, tweet_id: &str) -> String {
        format!(
            "{}{}{}",
            self.base_end_point, TWITTER_TWEET_BY_ID_ENDPOINT, tweet_id
        )
    }
}
