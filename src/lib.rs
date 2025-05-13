use chrono::DateTime;
use worker::*;

pub(crate) mod discord;
pub(crate) mod schlaugh;

#[event(scheduled)]
async fn scheduled(event: ScheduledEvent, _env: Env, _ctx: ScheduleContext) {
    console_error_panic_hook::set_once();
    console_log!("Scheduled function executed");
    console_log!("Cron: {}", event.cron());
    let current_date = DateTime::from_timestamp_millis(event.schedule() as i64).unwrap();
    console_log!("Current date: {}", &current_date);

    console_log!("Getting latest posts");
    let latest_posts = schlaugh::get_latest_posts("67c000111610bf329ab41598")
        .await
        .unwrap();
    let current_post = latest_posts
        .posts
        .iter()
        .find(|post| post.date == current_date.naive_utc().date());
    let Some(current_post) = current_post else {
        console_log!("No post found for current date");
        return;
    };

    console_log!("Current post: {:#?}", current_post);

    console_log!("Notifying with webhook");
    discord::notify_with_webhook("https://discord.com/api/webhooks/1371875768845598831/Xae_D8ABeSxDJ0-PaNz8JcFa_MCDHYfRPsxjbyfbZbb0E3YG2Q_G4zJYt4WfFds3PBya", &current_post, &latest_posts.author_info).await.unwrap();
    console_log!("Webhook notified");
}
