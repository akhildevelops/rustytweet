# rustytweet

<img src="https://www.rust-lang.org/static/images/rust-social.jpg" height=100>
<img src="https://upload.wikimedia.org/wikipedia/sco/9/9f/Twitter_bird_logo_2012.svg" height=100>

Library for twitter api based on api v2.

Currently the library accepts App-Access token i.e Bearer Token.

To generate access token for using twitter api. Follow this [twitter dev link](https://developer.twitter.com/en/docs/authentication/oauth-2-0/application-only).

Below are the few examples to access the twitter data.

Examples:

```rust
let twitter_client = TwitterClient::builder()
        .set_bearer_token("<bearer_token>".to_string())
        .build().unwrap();

let resp = twitter_client
        .search_recent_tweets("#nyc")
        .send();

```
