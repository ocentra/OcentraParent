use super::identifiers::{
    delete_request_ref, parent_timestamp, policy_ref, status_ref, tombstone_ref,
};
use super::*;

pub(super) fn sample_tombstones(timestamp: &ParentTimestamp) -> Vec<ParentOwnedSyncTombstoneRow> {
    vec![
        tombstone_row(
            "tombstone-none".to_string(),
            ParentOwnedSyncExportDataClass::ParentRule,
            ParentOwnedSyncTombstonePropagationState::NotRequested,
            None,
            "provider-status-google-drive-appdata-ready".to_string(),
            None,
            None,
        ),
        tombstone_row(
            "tombstone-pending".to_string(),
            ParentOwnedSyncExportDataClass::AuditEvent,
            ParentOwnedSyncTombstonePropagationState::Pending,
            Some("delete-request-audit".to_string()),
            "provider-status-dropbox-app-folder-partial-upload".to_string(),
            None,
            None,
        ),
        tombstone_row(
            "tombstone-propagated".to_string(),
            ParentOwnedSyncExportDataClass::GeneratedSummary,
            ParentOwnedSyncTombstonePropagationState::Propagated,
            Some("delete-request-summary".to_string()),
            "provider-status-onedrive-parent-selected-folder-wrong-account".to_string(),
            Some(timestamp.as_str()),
            None,
        ),
        tombstone_row(
            "tombstone-blocked".to_string(),
            ParentOwnedSyncExportDataClass::NotificationHistory,
            ParentOwnedSyncTombstonePropagationState::Blocked,
            Some("delete-request-notification".to_string()),
            "provider-status-icloud-drive-app-container-folder-unavailable".to_string(),
            None,
            Some("blocked-folder-unavailable".to_string()),
        ),
        tombstone_row(
            "tombstone-manual".to_string(),
            ParentOwnedSyncExportDataClass::DeviceRegistryEntry,
            ParentOwnedSyncTombstonePropagationState::ManualRequired,
            Some("delete-request-device-registry".to_string()),
            "provider-status-google-drive-picker-file-manual-required".to_string(),
            None,
            Some("manual-delete-confirmation-required".to_string()),
        ),
    ]
}

fn tombstone_row(
    tombstone_ref_value: String,
    data_class: ParentOwnedSyncExportDataClass,
    propagation_state: ParentOwnedSyncTombstonePropagationState,
    delete_request_ref_value: Option<String>,
    provider_status_ref_value: String,
    last_propagated_at: Option<&str>,
    blocked_reason_ref_value: Option<String>,
) -> ParentOwnedSyncTombstoneRow {
    ParentOwnedSyncTombstoneRow {
        tombstone_ref: tombstone_ref(tombstone_ref_value),
        data_class,
        propagation_state,
        delete_request_ref: delete_request_ref_value.map(delete_request_ref),
        provider_status_ref: status_ref(provider_status_ref_value),
        last_propagated_at: last_propagated_at.map(parent_timestamp),
        blocked_reason_ref: blocked_reason_ref_value.map(policy_ref),
        claim_safe: true,
    }
}
