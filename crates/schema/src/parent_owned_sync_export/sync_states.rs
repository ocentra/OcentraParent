use super::identifiers::{
    batch_ref, checksum_ref, conflict_ref, cursor_ref, parent_timestamp, policy_ref, signature_ref,
    status_ref,
};
use super::*;

pub(super) fn sample_sync_states() -> Vec<ParentOwnedSyncStateRow> {
    let mut rows = sample_sync_state_rows_primary();
    rows.extend(sample_sync_state_rows_secondary());
    rows
}

fn sample_sync_state_rows_primary() -> Vec<ParentOwnedSyncStateRow> {
    [
        ParentOwnedSyncStateRowInput {
            sync_state: ParentOwnedSyncState::Synced,
            provider_status_ref_value: "provider-status-google-drive-appdata-ready",
            cursor_ref_value: Some("cursor-synced"),
            batch_ref_value: Some("batch-synced"),
            manifest_integrity_state: ParentOwnedSyncManifestIntegrityState::Verified,
            checksum_ref_value: Some("checksum-synced"),
            signature_ref_value: Some("signature-synced"),
            last_successful_sync_at: Some("2026-06-28T18:40:00.000Z"),
            conflict_ref_value: None,
            retry_queue_ref_value: None,
            parent_action_required: false,
        },
        ParentOwnedSyncStateRowInput {
            sync_state: ParentOwnedSyncState::Stale,
            provider_status_ref_value: "provider-status-dropbox-parent-selected-folder-ready",
            cursor_ref_value: Some("cursor-stale"),
            batch_ref_value: Some("batch-stale"),
            manifest_integrity_state: ParentOwnedSyncManifestIntegrityState::Verified,
            checksum_ref_value: Some("checksum-stale"),
            signature_ref_value: Some("signature-stale"),
            last_successful_sync_at: Some("2026-06-28T18:20:00.000Z"),
            conflict_ref_value: None,
            retry_queue_ref_value: None,
            parent_action_required: false,
        },
        ParentOwnedSyncStateRowInput {
            sync_state: ParentOwnedSyncState::Missing,
            provider_status_ref_value: "provider-status-local-folder-ready",
            cursor_ref_value: None,
            batch_ref_value: None,
            manifest_integrity_state: ParentOwnedSyncManifestIntegrityState::Mismatch,
            checksum_ref_value: None,
            signature_ref_value: None,
            last_successful_sync_at: None,
            conflict_ref_value: None,
            retry_queue_ref_value: Some("retry-missing-manifest"),
            parent_action_required: true,
        },
        ParentOwnedSyncStateRowInput {
            sync_state: ParentOwnedSyncState::Conflict,
            provider_status_ref_value:
                "provider-status-onedrive-parent-selected-folder-wrong-account",
            cursor_ref_value: Some("cursor-conflict"),
            batch_ref_value: Some("batch-conflict"),
            manifest_integrity_state: ParentOwnedSyncManifestIntegrityState::Verified,
            checksum_ref_value: Some("checksum-conflict"),
            signature_ref_value: Some("signature-conflict"),
            last_successful_sync_at: None,
            conflict_ref_value: Some("conflict-parent-owned-sync-1"),
            retry_queue_ref_value: Some("retry-conflict"),
            parent_action_required: true,
        },
    ]
    .iter()
    .map(sync_state_row)
    .collect()
}

fn sample_sync_state_rows_secondary() -> Vec<ParentOwnedSyncStateRow> {
    [
        ParentOwnedSyncStateRowInput {
            sync_state: ParentOwnedSyncState::OfflineRetryPending,
            provider_status_ref_value: "provider-status-nas-folder-ready",
            cursor_ref_value: Some("cursor-offline-retry"),
            batch_ref_value: Some("batch-offline-retry"),
            manifest_integrity_state: ParentOwnedSyncManifestIntegrityState::Verified,
            checksum_ref_value: Some("checksum-offline-retry"),
            signature_ref_value: Some("signature-offline-retry"),
            last_successful_sync_at: Some("2026-06-28T18:10:00.000Z"),
            conflict_ref_value: None,
            retry_queue_ref_value: Some("retry-offline"),
            parent_action_required: false,
        },
        ParentOwnedSyncStateRowInput {
            sync_state: ParentOwnedSyncState::PartialOutage,
            provider_status_ref_value: "provider-status-dropbox-app-folder-partial-upload",
            cursor_ref_value: Some("cursor-partial-outage"),
            batch_ref_value: Some("batch-partial-outage"),
            manifest_integrity_state: ParentOwnedSyncManifestIntegrityState::Verified,
            checksum_ref_value: Some("checksum-partial-outage"),
            signature_ref_value: Some("signature-partial-outage"),
            last_successful_sync_at: Some("2026-06-28T18:00:00.000Z"),
            conflict_ref_value: None,
            retry_queue_ref_value: Some("retry-partial-outage"),
            parent_action_required: false,
        },
        ParentOwnedSyncStateRowInput {
            sync_state: ParentOwnedSyncState::ManualRequired,
            provider_status_ref_value: "provider-status-google-drive-picker-file-manual-required",
            cursor_ref_value: Some("cursor-manual"),
            batch_ref_value: Some("batch-manual"),
            manifest_integrity_state: ParentOwnedSyncManifestIntegrityState::Corrupt,
            checksum_ref_value: Some("checksum-manual"),
            signature_ref_value: Some("signature-manual"),
            last_successful_sync_at: None,
            conflict_ref_value: None,
            retry_queue_ref_value: Some("retry-manual-review"),
            parent_action_required: true,
        },
        ParentOwnedSyncStateRowInput {
            sync_state: ParentOwnedSyncState::NotStarted,
            provider_status_ref_value: "provider-status-disabled-disabled",
            cursor_ref_value: None,
            batch_ref_value: None,
            manifest_integrity_state: ParentOwnedSyncManifestIntegrityState::NotApplicable,
            checksum_ref_value: None,
            signature_ref_value: None,
            last_successful_sync_at: None,
            conflict_ref_value: None,
            retry_queue_ref_value: None,
            parent_action_required: false,
        },
    ]
    .iter()
    .map(sync_state_row)
    .collect()
}

struct ParentOwnedSyncStateRowInput<'a> {
    sync_state: ParentOwnedSyncState,
    provider_status_ref_value: &'a str,
    cursor_ref_value: Option<&'a str>,
    batch_ref_value: Option<&'a str>,
    manifest_integrity_state: ParentOwnedSyncManifestIntegrityState,
    checksum_ref_value: Option<&'a str>,
    signature_ref_value: Option<&'a str>,
    last_successful_sync_at: Option<&'a str>,
    conflict_ref_value: Option<&'a str>,
    retry_queue_ref_value: Option<&'a str>,
    parent_action_required: bool,
}

fn sync_state_row(input: &ParentOwnedSyncStateRowInput<'_>) -> ParentOwnedSyncStateRow {
    ParentOwnedSyncStateRow {
        sync_state: input.sync_state,
        provider_status_ref: status_ref(input.provider_status_ref_value),
        cursor_ref: input.cursor_ref_value.map(cursor_ref),
        batch_ref: input.batch_ref_value.map(batch_ref),
        manifest_integrity_state: input.manifest_integrity_state,
        manifest_checksum_ref: input.checksum_ref_value.map(checksum_ref),
        manifest_signature_ref: input.signature_ref_value.map(signature_ref),
        last_successful_sync_at: input.last_successful_sync_at.map(parent_timestamp),
        conflict_ref: input.conflict_ref_value.map(conflict_ref),
        retry_queue_ref: input.retry_queue_ref_value.map(policy_ref),
        parent_action_required: input.parent_action_required,
        claim_safe: true,
    }
}
