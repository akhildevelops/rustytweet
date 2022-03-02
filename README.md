# rustytweet

<span>
<img src="https://www.rust-lang.org/static/images/rust-social.jpg" height=100>
<img src="https://upload.wikimedia.org/wikipedia/sco/9/9f/Twitter_bird_logo_2012.svg" height=100>
</span>
</br>

### Library for Twitter V2 API.

Currently the library accepts App-Access token i.e Bearer Token.

To generate access token for using twitter api. Follow this [twitter dev link](https://developer.twitter.com/en/docs/authentication/oauth-2-0/application-only).

Below are the few examples to access the twitter data.

Examples:

#### Below is a basic example for fetching tweets with hashtag **#nyc**

```rust
let twitter_client = TwitterClient::builder()
        .set_bearer_token("<bearer_token>".to_string())
        .build().unwrap();

let resp = twitter_client
        .search_recent_tweets("#nyc")
        .send();

```

For more Tweet related content about author of the tweet, location, etc..,. use expansions. Refer to [Twitter Expansions](https://developer.twitter.com/en/docs/twitter-api/expansions)

```rust
let twitter_client = TwitterClient::builder()
        .set_bearer_token("<bearer_token>".to_string())
        .build();

    let media_expansion = Expansion::User(&["description", "created_at", "location"]);
    let tweet_expansion = Expansion::Tweet(&[
        "author_id",
        "created_at",
        "in_reply_to_user_id",
        "referenced_tweets",
    ]);
    let resp = twitter_client
        .unwrap()
        .search_recent_tweets("#nyc")
        .expansion(&[media_expansion, tweet_expansion])
        .send();
```
