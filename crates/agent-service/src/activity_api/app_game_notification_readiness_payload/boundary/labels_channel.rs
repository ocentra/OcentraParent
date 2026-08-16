use ocentra_parent_agent_protocol::app_game_notification_status::AppGameNotificationProviderChannel;

use super::super::super::constants::PROVIDER_STATUS_UNAVAILABLE;

pub(super) fn provider_channel_label(status: AppGameNotificationProviderChannel) -> String {
    match status {
        AppGameNotificationProviderChannel::Unavailable => PROVIDER_STATUS_UNAVAILABLE,
    }
    .to_string()
}
