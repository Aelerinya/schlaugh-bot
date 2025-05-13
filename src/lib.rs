use worker::*;
use chrono::DateTime;

mod schlaugh;

#[event(scheduled)]
async fn scheduled(
    event: ScheduledEvent,
    _env: Env,
    _ctx: ScheduleContext,
) {
    console_error_panic_hook::set_once();
    console_log!("Scheduled function executed");
    let current_date = DateTime::from_timestamp_millis(event.schedule() as i64).unwrap();
    console_log!("Current date: {}", &current_date);
    let posts = schlaugh::get_latest_posts("67c000111610bf329ab41598").await.unwrap();
    let current_post = posts.posts.iter().find(|post| post.date == current_date.naive_utc().date());
    if let Some(post) = current_post {
        console_log!("Current post: {:#?}", post);
    } else {
        console_log!("No post found for current date");
    }
    
    // // get date in format YYYY-MM-DD
    // let current_date = js_date.get_time();
}