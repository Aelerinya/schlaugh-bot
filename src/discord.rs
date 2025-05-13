use serde_json::json;
use crate::schlaugh;
use html2md::parse_html;

const MAX_DESCRIPTION_LENGTH: usize = 2000;


pub async fn notify_with_webhook(webhook_url: &str, post: &schlaugh::Post, author: &schlaugh::AuthorInfo) -> Result<(), reqwest::Error> {
    let post_url = format!("https://schlaugh.com/~/{}", post.post_id);
    let client = reqwest::Client::new();
    let markdown_body = parse_html(&post.body);
    let description = if markdown_body.len() > MAX_DESCRIPTION_LENGTH {
        format!("{}[...]\n\n[Read more]({})", &markdown_body[..MAX_DESCRIPTION_LENGTH], post_url)
    } else {
        markdown_body
    };

    let response = client.post(webhook_url)
        .json(&json!({
            "username": "schlaugh-bot",
            "avatar_url": "https://www.schlaugh.com/favicon.png",
            "embeds": [
                {
                    "title": if post.title.is_empty() { post.date.format("%Y-%m-%d").to_string() } else { post.title.clone() },
                    // "type": "article",
                    "description": description,
                    "url": post_url,
                    "author": {
                        "name": author.author,
                        "icon_url": author.author_pic,
                        "url": format!("https://schlaugh.com/{}", author.author),
                    },
                    "footer": {
                        "text": "Schlaugh",
                        "icon_url": "https://www.schlaugh.com/favicon.png",
                    },
                    // "color": 0x0099ff,
                    "provider": {
                        "name": "schlaugh-bot",
                        "url": "https://github.com/Aelerinya/schlaugh-bot",
                    }
                }
            ]
        }))
        .send()
        .await?;
    println!("Response: {}", response.text().await?);
    Ok(())
}


#[cfg(test)]
#[tokio::test]
async fn test_notify_with_webhook() {
    let response = schlaugh::get_latest_posts("67c000111610bf329ab41598").await.unwrap();
    notify_with_webhook("https://discord.com/api/webhooks/1371875768845598831/Xae_D8ABeSxDJ0-PaNz8JcFa_MCDHYfRPsxjbyfbZbb0E3YG2Q_G4zJYt4WfFds3PBya", &response.posts[2], &response.author_info).await.unwrap();
}