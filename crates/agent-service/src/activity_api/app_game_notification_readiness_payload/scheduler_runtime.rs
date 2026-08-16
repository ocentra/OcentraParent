#[path = "boundary/scheduler.rs"]
pub(super) mod boundary;
#[path = "boundary/scheduler_constants.rs"]
pub(super) mod scheduler_constants;

pub(super) fn load_verified_notification_preflight(
) -> std::io::Result<Option<boundary::VerifiedNotificationPreflight>> {
    boundary::load_verified_notification_preflight()
}
