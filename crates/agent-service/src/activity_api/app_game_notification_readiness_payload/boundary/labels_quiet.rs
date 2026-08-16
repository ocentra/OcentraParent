use ocentra_parent_agent_protocol::app_game_notification_status::AppGameNotificationQuietHoursDecision;

use super::super::super::constants::{
    PROVIDER_STATUS_MANUAL_REQUIRED, PROVIDER_STATUS_UNAVAILABLE, QUIET_HOURS_ALLOW,
};

pub(super) fn quiet_hours_decision_label(status: AppGameNotificationQuietHoursDecision) -> String {
    match status {
        AppGameNotificationQuietHoursDecision::Allow => QUIET_HOURS_ALLOW,
        AppGameNotificationQuietHoursDecision::ManualRequired => PROVIDER_STATUS_MANUAL_REQUIRED,
        AppGameNotificationQuietHoursDecision::Unavailable => PROVIDER_STATUS_UNAVAILABLE,
    }
    .to_string()
}
