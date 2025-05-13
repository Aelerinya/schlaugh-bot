use serde::Deserialize;
use chrono::NaiveDate;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Post {
    pub body: String,
    pub tags: serde_json::Value,
    pub title: String,
    pub url: String,
    #[serde(rename = "post_id")]
    pub post_id: String,
    pub date: NaiveDate,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct AuthorInfo {
    pub bio: Option<String>,
    pub key: Option<String>,
    #[serde(rename = "_id")]
    pub id: String,
    pub author: String,
    #[serde(rename = "authorPic")]
    pub author_pic: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Response {
    pub error: bool,
    pub posts: Vec<Post>,
    #[serde(rename = "authorData")]
    pub author_data: AuthorInfo,
    pub pages: i32,
    #[serde(rename = "authorInfo")]
    pub author_info: AuthorInfo,
}

pub async fn get_latest_posts(user: &str) -> Result<Response, reqwest::Error> {
    let client = reqwest::Client::new();
    client.post("https://www.schlaugh.com/getPosts")
        .body(format!("{{\"author\":\"{user}\",\"page\":0,\"postCode\":\"TFFF\",\"needAuthorInfo\":true}}"))
        .header("Content-Type", "application/json")
        .send()
        .await?
        .json::<Response>()
        .await
}

#[cfg(test)]
#[tokio::test]
async fn test_get_latest_posts() {
    let response = get_latest_posts("67c000111610bf329ab41598").await.unwrap();
    println!("Latest post: {:#?}", response);
}