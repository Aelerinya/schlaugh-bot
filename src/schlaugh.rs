use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Post {
    body: String,
    tags: serde_json::Value,
    title: String,
    url: String,
    #[serde(rename = "post_id")]
    post_id: String,
    date: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct AuthorInfo {
    bio: Option<String>,
    key: Option<String>,
    #[serde(rename = "_id")]
    id: String,
    author: String,
    #[serde(rename = "authorPic")]
    author_pic: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Response {
    error: bool,
    posts: Vec<Post>,
    #[serde(rename = "authorData")]
    author_data: AuthorInfo,
    pages: i32,
    #[serde(rename = "authorInfo")]
    author_info: AuthorInfo,
}

async fn get_latest_posts(user: &str) -> Result<Response, reqwest::Error> {
    let client = reqwest::Client::new();
    client.post("https://www.schlaugh.com/getPosts")
        .body(format!("{{\"author\":\"{user}\",\"page\":0,\"postCode\":\"TFFF\",\"needAuthorInfo\":true}}"))
        .header("Content-Type", "application/json")
        .send()
        .await?
        .json::<Response>()
        .await
}

#[tokio::test]
async fn test_get_latest_posts() {
    let response = get_latest_posts("67c000111610bf329ab41598").await.unwrap();
    println!("Latest post: {:#?}", response);
}