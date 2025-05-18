use crate::schlaugh;
use html2md::parse_html;
use serde_json::json;

const MAX_DESCRIPTION_LENGTH: usize = 2000;

pub fn create_embed(post: &schlaugh::Post, author: &schlaugh::AuthorInfo) -> serde_json::Value {
    let post_url = format!("https://schlaugh.com/~/{}", post.post_id);

    // Replace double newlines with br tags
    let body_with_br = post.body.replace("\n", "<br />");
    let markdown_body = parse_html(&body_with_br);

    let description = if markdown_body.len() > MAX_DESCRIPTION_LENGTH {
        format!(
            "{}[...]\n\n[Read more]({})",
            &markdown_body[..MAX_DESCRIPTION_LENGTH],
            post_url
        )
    } else {
        markdown_body
    };

    json!({
        "title": if post.title.is_empty() { post.date.format("%Y-%m-%d").to_string() } else { post.title.clone() },
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
    })
}

pub async fn notify_with_webhook(
    webhook_url: &str,
    embeds: Vec<serde_json::Value>,
) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();

    let response = client
        .post(webhook_url)
        .json(&json!({
            "username": "schlaugh-bot",
            "avatar_url": "https://www.schlaugh.com/favicon.png",
            "embeds": embeds,
        }))
        .send()
        .await?;
    println!("Response: {}", response.text().await?);
    Ok(())
}

#[cfg(test)]
#[tokio::test]
async fn test_notify_with_webhook() {
    dotenv::from_filename(".dev.vars").ok();

    let response = schlaugh::get_latest_posts("67c000111610bf329ab41598")
        .await
        .unwrap();
    dbg!(&response.posts[0]);
    let embed = create_embed(&response.posts[0], &response.author_info);
    notify_with_webhook(
        &std::env::var("DISCORD_WEBHOOK_URL").expect("Missing DISCORD_WEBHOOK_URL"),
        vec![embed],
    )
    .await
    .unwrap();
}
