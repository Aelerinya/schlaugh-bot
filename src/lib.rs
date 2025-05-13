use worker::*;

mod schlaugh;

#[event(scheduled)]
async fn scheduled(
    _envent: ScheduledEvent,
    _env: Env,
    _ctx: ScheduleContext,
) {
    console_error_panic_hook::set_once();
    console_log!("Scheduled function executed");
}