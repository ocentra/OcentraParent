#[path = "labels_notification.rs"]
mod notification;
#[path = "labels_preference.rs"]
mod preference;

use ocentra_parent_agent_protocol::app_game_notification_status::{
    AppGameNotificationParentPreferenceState, AppGameNotificationPreferenceDeliveryResultState,
    AppGameNotificationProviderChannel, AppGameNotificationQuietHoursDecision,
};
use ocentra_parent_agent_protocol::notification_provider_status_boundary::V08NotificationProviderStatus;

pub(super) fn provider_status_label(status: V08NotificationProviderStatus) -> String {
    notification::provider_status_label(status)
}

pub(super) fn preference_delivery_result_label(
    status: AppGameNotificationPreferenceDeliveryResultState,
) -> String {
    preference::preference_delivery_result_label(status)
}

pub(super) fn parent_preference_state_label(
    status: AppGameNotificationParentPreferenceState,
) -> String {
    preference::parent_preference_state_label(status)
}

pub(super) fn quiet_hours_decision_label(status: AppGameNotificationQuietHoursDecision) -> String {
    preference::quiet_hours_decision_label(status)
}

pub(super) fn provider_channel_label(status: AppGameNotificationProviderChannel) -> String {
    preference::provider_channel_label(status)
}
