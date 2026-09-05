#[path = "boundary/scheduler.rs"]
pub(super) mod boundary;
#[path = "boundary/scheduler_constants.rs"]
pub(super) mod scheduler_constants;

pub(super) fn load_verified_notification_preflight(
) -> std::io::Result<Option<boundary::VerifiedNotificationPreflight>> {
    boundary::load_verified_notification_preflight()
}

pub(super) fn load_verified_notification_preflight_from_activity_db_path(
    activity_db_path: &std::path::Path,
) -> std::io::Result<Option<boundary::VerifiedNotificationPreflight>> {
    boundary::load_verified_notification_preflight_from_activity_db_path(activity_db_path)
}
