# rustytweet

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
