use rustytweet::expansion::Expansion;

use rustytweet::TwitterClient;

use std::collections::HashMap;
fn main() {
    let mut hm = HashMap::new();
    hm.insert("max_results".to_string(), "100".to_string());

    let twitter_client = TwitterClient::builder()
        .set_bearer_token("AAAAAAAAAAAAAAAAAAAAACuFZQEAAAAAwKKMQk7xK2rhCpTVctpBZqhzVxI%3DZEd7d5kCShK6kUtSJ7u0leE7XXv5i8v2Vgryc7jUihcE5yEQ0s".to_string())
        .set_default_query_params(hm)
        .build(); // Interesting as it gets consumed build method. Fuck YAAAAAAAAAAAAAAA!!!!!!!!!!! it is not accepted when only a shared reference is passed.

    let media_expansion = Expansion::User(&["description", "created_at"]);
    let tweet_expansion = Expansion::Tweet(&[
        "author_id",
        "created_at",
        "in_reply_to_user_id",
        "referenced_tweets",
    ]);
    let resp = twitter_client
        .search_recent_tweets("#nyc -is:retweet -is:reply is:verified")
        .expansion(&[media_expansion, tweet_expansion])
        .send();

    println!("{:?}", resp.unwrap())
}
