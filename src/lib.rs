use chrono::DateTime;
use serde::Deserialize;
use worker::*;

pub(crate) mod discord;
pub(crate) mod schlaugh;

#[derive(Deserialize, Debug)]
struct SchlaughUsers {
    ids: Vec<String>,
}

#[event(scheduled)]
async fn scheduled(event: ScheduledEvent, env: Env, _ctx: ScheduleContext) {
    console_error_panic_hook::set_once();

    let discord_webhook_url = env.var("DISCORD_WEBHOOK_URL").unwrap().to_string();
    let schlaugh_users: SchlaughUsers = env.object_var("SCHLAUGH_USERS").unwrap();

    console_log!("Scheduled function executed");
    console_log!("Cron: {}", event.cron());
    let current_date = DateTime::from_timestamp_millis(event.schedule() as i64)
        .unwrap()
        .naive_utc()
        .date();
    console_log!("Current date: {}", &current_date);

    let mut posts_embeds = Vec::new();
    for user in schlaugh_users.ids {
        console_log!("Getting latest posts for user: {}", user);
        let latest_posts = schlaugh::get_latest_posts(&user).await.unwrap();
        let current_post = latest_posts
            .posts
            .iter()
            .find(|post| post.date == current_date);
        let Some(current_post) = current_post else {
            console_log!("No post found for current date");
            continue;
        };

        console_log!("Current post: {:#?}", current_post);
        posts_embeds.push(discord::create_embed(
            current_post,
            &latest_posts.author_info,
        ));
    }

    console_log!("Notifying with webhook");
    discord::notify_with_webhook(&discord_webhook_url, posts_embeds)
        .await
        .unwrap();
    console_log!("Webhook notified");
}
