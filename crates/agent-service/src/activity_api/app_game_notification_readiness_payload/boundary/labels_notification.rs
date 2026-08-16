use ocentra_parent_agent_protocol::notification_provider_status_boundary::V08NotificationProviderStatus;

use super::super::constants::{
    PROVIDER_STATUS_DELIVERED, PROVIDER_STATUS_FAILED, PROVIDER_STATUS_MANUAL_REQUIRED,
    PROVIDER_STATUS_QUEUED, PROVIDER_STATUS_UNAVAILABLE,
};

pub(super) fn provider_status_label(status: V08NotificationProviderStatus) -> String {
    match status {
        V08NotificationProviderStatus::Queued => PROVIDER_STATUS_QUEUED,
        V08NotificationProviderStatus::Delivered => PROVIDER_STATUS_DELIVERED,
        V08NotificationProviderStatus::Failed => PROVIDER_STATUS_FAILED,
        V08NotificationProviderStatus::Unavailable => PROVIDER_STATUS_UNAVAILABLE,
        V08NotificationProviderStatus::ManualRequired => PROVIDER_STATUS_MANUAL_REQUIRED,
    }
    .to_string()
}
