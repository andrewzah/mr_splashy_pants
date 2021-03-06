![Rust](https://github.com/tobymurray/mr_splashy_pants/workflows/Rust/badge.svg)
[![Documentation](https://img.shields.io/badge/documentation-available-green.svg)](https://docs.rs/crate/mr_splashy_pants/)
[![Crate](https://img.shields.io/crates/v/mr_splashy_pants.svg)](https://crates.io/crates/mr_splashy_pants)

# What is this?
WIP Rust bindings for the Reddit API

# This is a WIP, you likely won't find it particularly useful

# Set up
Follow https://github.com/reddit-archive/reddit/wiki/OAuth2 for set up instructions. You can use https://tobymurray.github.io/reddit-auth-generator/ to help generate an auth string.

# Use
Set up a script with access to a Reddit account, collect the access token, the client ID, and the client secret. Once you have that, get a refresh token. Once you have that you can do:

```
// pants is mutable so the refresh token can be updated
let mut pants = Pants::new(
    USER_AGENT,
    "<access-token>",
    "<refresh_token>",
    "<client-id>",
    "<client-secret>",
);
```
For example, if you're using dotenv and reading values from the environment:
```
// pants is mutable so the refresh token can be updated
let mut pants = Pants::new(
    USER_AGENT,
    env::var("ACCESS_TOKEN").unwrap(),
    &env::var("REFRESH_TOKEN").unwrap(),
    &env::var("CLIENT_ID").unwrap(),
    &env::var("CLIENT_SECRET").unwrap(),
);
```
Then you can invoke things, e.g:

```
pants.me()
```

If your refresh token expires, it should automatically refresh.

Currently implemented with (partially) structured response:

**Account:**
- GET [/api/v1/me](https://www.reddit.com/dev/api#GET_api_v1_me)
- GET [/api/v1/me/karma](https://www.reddit.com/dev/api#GET_api_v1_me_karma)
- GET [/api/v1/me/prefs](https://www.reddit.com/dev/api#GET_api_v1_me_prefs)
- GET [/prefs/friends](https://www.reddit.com/dev/api#GET_prefs_friends)

Currently kind of implemented (no query parameters), with JSON response:

**Account:**
- GET [/api/v1/me/trophies](https://www.reddit.com/dev/api#GET_api_v1_me_trophies)
- GET [/prefs/blocked](https://www.reddit.com/dev/api#GET_prefs_blocked)
- GET [/prefs/messaging](https://www.reddit.com/dev/api#GET_prefs_messaging)
- GET [/prefs/trusted](https://www.reddit.com/dev/api#GET_prefs_trusted)
- GET [/api/v1/me/friends](https://www.reddit.com/dev/api#GET_api_v1_me_friends)
- GET [/api/v1/me/blocked](https://www.reddit.com/dev/api#GET_api_v1_me_blocked)

**Listing**

- GET [/api/trending_subreddits](https://www.reddit.com/dev/api#GET_api_trending_subreddits)
- GET [/best](https://www.reddit.com/dev/api#GET_best)
- GET [/by_id/names](https://www.reddit.com/dev/api#GET_by_id_{names})
- GET [/comments/article](https://www.reddit.com/dev/api#GET_comments_{article})
- GET [/controversial](https://www.reddit.com/dev/api#GET_controversial)
- GET [/r/{subreddit}/controversial](https://www.reddit.com/dev/api#GET_controversial)
- GET [/duplicates/article](https://www.reddit.com/dev/api#GET_duplicates_{article})
- GET [/hot](https://www.reddit.com/dev/api#GET_hot)
- GET [/r/{subreddit}/hot](https://www.reddit.com/dev/api#GET_hot)
- GET [/new](https://www.reddit.com/dev/api#GET_new)
- GET [/r/{subreddit}/new](https://www.reddit.com/dev/api#GET_new)
- GET [/random](https://www.reddit.com/dev/api#GET_random)
- GET [/r/{subreddit}/random](https://www.reddit.com/dev/api#GET_random)
- GET [/rising](https://www.reddit.com/dev/api#GET_rising)
- GET [/r/{subreddit}/rising](https://www.reddit.com/dev/api#GET_rising)
- GET [/top](https://www.reddit.com/dev/api#GET_top)
- GET [/r/{subreddit}/top](https://www.reddit.com/dev/api#GET_top)

**Links and Comments**
- POST [/api/submit](https://www.reddit.com/dev/api#POST_api_submit)
- POST [/api/del](https://www.reddit.com/dev/api#POST_api_del)

To submit a post to Reddit:

```
// Build the submission 
let request_body = links_and_comments::ApiSubmit {
    ad: "".to_string(),
    api_type: "".to_string(),
    app: "".to_string(),
    collection_id: "".to_string(),
    event_end: "".to_string(),
    event_start: "".to_string(),
    event_tz: "".to_string(),
    extension: "".to_string(),
    flair_id: "".to_string(),
    flair_text: "".to_string(),
    g_recaptcha_response: "".to_string(),
    kind: "self".to_string(),
    nsfw: "".to_string(),
    resubmit: "".to_string(),
    richtext_json: "".to_string(),
    sendreplies: "".to_string(),
    spoiler: "".to_string(),
    sr: "name_of_subreddit".to_string(),
    text: "Here's an example of the post's body".to_string(),
    title: "This is the title of the post".to_string(),
    uh: "".to_string(),
    url: "".to_string(),
    video_poster_url: "".to_string(),
};


// then submit the post
let submission_name = pants.submit(request_body).await {
    Ok(response) => {
        println!("Response to submit is: {}", serde_json::to_string_pretty(&response).unwrap());
        response.json.data.name
    },
    Err(e) => panic!("An error ocurred: {}", e),
};

// remove it if you'd like
let delete_request_body = links_and_comments::ApiDel { id: submission_name };

pants.del(delete_request_body).await;
```

Streaming support for:

**Disclaimer:** This implementation of streaming is not compatible with very high traffic subreddits. If more than 25 posts are submitted within any 30 second period, this streaming method will miss some.

- GET [/r/{subreddit}/new](https://www.reddit.com/dev/api#GET_new)

```
use futures_util::pin_mut;
use futures_util::stream::StreamExt;
...
let stream = pants.stream_subreddit_new("testingground4bots");
pin_mut!(stream);

while let Some(value) = tokio_test::block_on(stream.next()) {
    match value {
        Ok(data) => {
            println!("New post: {}", data);
        }
        Err(e) => {
            // Note, this can get noisy if the failure persists
            println!("Encountered an error: {}", e);
        }
    }
}
```

All other APIs are not implemented

# Path to version 1.0
As it stands, I'm making arbitrary changes to the library to support whatever I happen to be working on at the time. Additionally, I'm new to Rust, so I generally don't have a good sense of what I'm doing. To that end, this library's interface should be considered unstable and often _not good_. Until 1.0, it's entirely possible that every new release will introduce breaking changes. I'm not clear on where I want to end up with this library - should it be a "low level" API wrapper that just provides types to the Reddit API? Should it try and provide some usability improvement over the raw API? Should it focus on streamlining the most common use cases for bots to interact with Reddit?

Until I have some clarity myself around these questions, this library should be expected to change dramatically.
