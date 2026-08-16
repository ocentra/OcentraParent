use ocentra_parent_agent_protocol::app_game_notification_status::{
    AppGameNotificationParentPreferenceState, AppGameNotificationPreferenceDeliveryResultState,
};

use super::super::constants::{
    DELIVERY_RESULT_NOT_SENT, PREFERENCE_STATE_CHANNEL_DISABLED,
    PREFERENCE_STATE_MANUAL_SETUP_REQUIRED, PROVIDER_STATUS_MANUAL_REQUIRED,
    PROVIDER_STATUS_UNAVAILABLE,
};

#[path = "labels_channel.rs"]
mod channel;
#[path = "labels_quiet.rs"]
mod quiet;

pub(super) fn preference_delivery_result_label(
    status: AppGameNotificationPreferenceDeliveryResultState,
) -> String {
    match status {
        AppGameNotificationPreferenceDeliveryResultState::NotSent => DELIVERY_RESULT_NOT_SENT,
        AppGameNotificationPreferenceDeliveryResultState::ManualRequired => {
            PROVIDER_STATUS_MANUAL_REQUIRED
        }
        AppGameNotificationPreferenceDeliveryResultState::Unavailable => {
            PROVIDER_STATUS_UNAVAILABLE
        }
    }
    .to_string()
}

pub(super) fn parent_preference_state_label(
    status: AppGameNotificationParentPreferenceState,
) -> String {
    match status {
        AppGameNotificationParentPreferenceState::ChannelDisabled => {
            PREFERENCE_STATE_CHANNEL_DISABLED
        }
        AppGameNotificationParentPreferenceState::ManualSetupRequired => {
            PREFERENCE_STATE_MANUAL_SETUP_REQUIRED
        }
        AppGameNotificationParentPreferenceState::Unavailable => PROVIDER_STATUS_UNAVAILABLE,
    }
    .to_string()
}

pub(super) fn quiet_hours_decision_label(
    status: ocentra_parent_agent_protocol::app_game_notification_status::AppGameNotificationQuietHoursDecision,
) -> String {
    quiet::quiet_hours_decision_label(status)
}

pub(super) fn provider_channel_label(
    status: ocentra_parent_agent_protocol::app_game_notification_status::AppGameNotificationProviderChannel,
) -> String {
    channel::provider_channel_label(status)
}
