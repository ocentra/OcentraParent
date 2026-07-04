use super::identifiers::{provider_id, provider_ref, status_ref};
use super::*;

pub(super) fn sample_provider_statuses(
    timestamp: &ParentTimestamp,
) -> Vec<ParentOwnedSyncProviderStatusRow> {
    let mut rows = sample_provider_statuses_cloud(timestamp);
    rows.extend(sample_provider_statuses_icloud(timestamp));
    rows.extend(sample_provider_statuses_storage(timestamp));
    rows
}

fn sample_provider_statuses_cloud(
    timestamp: &ParentTimestamp,
) -> Vec<ParentOwnedSyncProviderStatusRow> {
    [
        ParentOwnedSyncProviderStatusInput {
            provider_mode: ParentOwnedSyncProviderMode::GoogleDriveAppdata,
            provider_status: ParentOwnedSyncProviderStatus::Ready,
            destination_ownership:
                ParentOwnedSyncExportDestinationOwnership::ParentOwnedExternalStorage,
            account_ref: Some("account-google-drive-appdata"),
            folder_ref: Some("folder-google-drive-appdata"),
            revocation_ref: None,
            disconnect_visibility_state: ParentOwnedSyncDisconnectVisibilityState::NotDisconnected,
            delete_visibility_state: ParentOwnedSyncDeleteVisibilityState::NotRequested,
            timestamp,
        },
        ParentOwnedSyncProviderStatusInput {
            provider_mode: ParentOwnedSyncProviderMode::GoogleDrivePickerFile,
            provider_status: ParentOwnedSyncProviderStatus::ManualRequired,
            destination_ownership:
                ParentOwnedSyncExportDestinationOwnership::ParentOwnedExternalStorage,
            account_ref: Some("account-google-drive-picker"),
            folder_ref: Some("folder-google-drive-picker"),
            revocation_ref: None,
            disconnect_visibility_state: ParentOwnedSyncDisconnectVisibilityState::ManualRequired,
            delete_visibility_state: ParentOwnedSyncDeleteVisibilityState::ManualRequired,
            timestamp,
        },
        ParentOwnedSyncProviderStatusInput {
            provider_mode: ParentOwnedSyncProviderMode::OnedriveApproot,
            provider_status: ParentOwnedSyncProviderStatus::Revoked,
            destination_ownership:
                ParentOwnedSyncExportDestinationOwnership::ParentOwnedExternalStorage,
            account_ref: Some("account-onedrive-approot"),
            folder_ref: Some("folder-onedrive-approot"),
            revocation_ref: Some("revoked-onedrive-approot"),
            disconnect_visibility_state: ParentOwnedSyncDisconnectVisibilityState::NotDisconnected,
            delete_visibility_state: ParentOwnedSyncDeleteVisibilityState::NotRequested,
            timestamp,
        },
        ParentOwnedSyncProviderStatusInput {
            provider_mode: ParentOwnedSyncProviderMode::OnedriveParentSelectedFolder,
            provider_status: ParentOwnedSyncProviderStatus::WrongAccount,
            destination_ownership:
                ParentOwnedSyncExportDestinationOwnership::ParentOwnedExternalStorage,
            account_ref: Some("account-onedrive-selected"),
            folder_ref: Some("folder-onedrive-selected"),
            revocation_ref: None,
            disconnect_visibility_state: ParentOwnedSyncDisconnectVisibilityState::NotDisconnected,
            delete_visibility_state: ParentOwnedSyncDeleteVisibilityState::DeleteVisible,
            timestamp,
        },
    ]
    .iter()
    .map(provider_status_row)
    .collect()
}

fn sample_provider_statuses_icloud(
    timestamp: &ParentTimestamp,
) -> Vec<ParentOwnedSyncProviderStatusRow> {
    [
        ParentOwnedSyncProviderStatusInput {
            provider_mode: ParentOwnedSyncProviderMode::IcloudDriveAppContainer,
            provider_status: ParentOwnedSyncProviderStatus::FolderUnavailable,
            destination_ownership:
                ParentOwnedSyncExportDestinationOwnership::ParentOwnedExternalStorage,
            account_ref: Some("account-icloud-container"),
            folder_ref: Some("folder-icloud-container"),
            revocation_ref: None,
            disconnect_visibility_state: ParentOwnedSyncDisconnectVisibilityState::NotDisconnected,
            delete_visibility_state: ParentOwnedSyncDeleteVisibilityState::DeleteFailed,
            timestamp,
        },
        ParentOwnedSyncProviderStatusInput {
            provider_mode: ParentOwnedSyncProviderMode::IcloudDriveParentSelectedLocation,
            provider_status: ParentOwnedSyncProviderStatus::Disconnected,
            destination_ownership:
                ParentOwnedSyncExportDestinationOwnership::ParentOwnedExternalStorage,
            account_ref: Some("account-icloud-location"),
            folder_ref: Some("folder-icloud-location"),
            revocation_ref: None,
            disconnect_visibility_state:
                ParentOwnedSyncDisconnectVisibilityState::DisconnectVisible,
            delete_visibility_state: ParentOwnedSyncDeleteVisibilityState::NotRequested,
            timestamp,
        },
        ParentOwnedSyncProviderStatusInput {
            provider_mode: ParentOwnedSyncProviderMode::DropboxAppFolder,
            provider_status: ParentOwnedSyncProviderStatus::PartialUpload,
            destination_ownership:
                ParentOwnedSyncExportDestinationOwnership::ParentOwnedExternalStorage,
            account_ref: Some("account-dropbox-app"),
            folder_ref: Some("folder-dropbox-app"),
            revocation_ref: None,
            disconnect_visibility_state: ParentOwnedSyncDisconnectVisibilityState::NotDisconnected,
            delete_visibility_state: ParentOwnedSyncDeleteVisibilityState::DeleteConfirmed,
            timestamp,
        },
    ]
    .iter()
    .map(provider_status_row)
    .collect()
}

fn sample_provider_statuses_storage(
    timestamp: &ParentTimestamp,
) -> Vec<ParentOwnedSyncProviderStatusRow> {
    [
        ParentOwnedSyncProviderStatusInput {
            provider_mode: ParentOwnedSyncProviderMode::DropboxParentSelectedFolder,
            provider_status: ParentOwnedSyncProviderStatus::Ready,
            destination_ownership:
                ParentOwnedSyncExportDestinationOwnership::ParentOwnedExternalStorage,
            account_ref: Some("account-dropbox-selected"),
            folder_ref: Some("folder-dropbox-selected"),
            revocation_ref: None,
            disconnect_visibility_state: ParentOwnedSyncDisconnectVisibilityState::NotDisconnected,
            delete_visibility_state: ParentOwnedSyncDeleteVisibilityState::NotRequested,
            timestamp,
        },
        ParentOwnedSyncProviderStatusInput {
            provider_mode: ParentOwnedSyncProviderMode::NasFolder,
            provider_status: ParentOwnedSyncProviderStatus::Ready,
            destination_ownership:
                ParentOwnedSyncExportDestinationOwnership::ParentOwnedExternalStorage,
            account_ref: Some("account-nas-folder"),
            folder_ref: Some("folder-nas-folder"),
            revocation_ref: None,
            disconnect_visibility_state: ParentOwnedSyncDisconnectVisibilityState::NotDisconnected,
            delete_visibility_state: ParentOwnedSyncDeleteVisibilityState::NotRequested,
            timestamp,
        },
        ParentOwnedSyncProviderStatusInput {
            provider_mode: ParentOwnedSyncProviderMode::LocalFolder,
            provider_status: ParentOwnedSyncProviderStatus::Ready,
            destination_ownership: ParentOwnedSyncExportDestinationOwnership::ParentDeviceLocal,
            account_ref: Some("account-local-folder"),
            folder_ref: Some("folder-local-folder"),
            revocation_ref: None,
            disconnect_visibility_state: ParentOwnedSyncDisconnectVisibilityState::NotDisconnected,
            delete_visibility_state: ParentOwnedSyncDeleteVisibilityState::NotRequested,
            timestamp,
        },
        ParentOwnedSyncProviderStatusInput {
            provider_mode: ParentOwnedSyncProviderMode::Disabled,
            provider_status: ParentOwnedSyncProviderStatus::Disabled,
            destination_ownership: ParentOwnedSyncExportDestinationOwnership::ParentDeviceLocal,
            account_ref: None,
            folder_ref: None,
            revocation_ref: None,
            disconnect_visibility_state: ParentOwnedSyncDisconnectVisibilityState::NotDisconnected,
            delete_visibility_state: ParentOwnedSyncDeleteVisibilityState::NotRequested,
            timestamp,
        },
    ]
    .iter()
    .map(provider_status_row)
    .collect()
}

struct ParentOwnedSyncProviderStatusInput<'a> {
    provider_mode: ParentOwnedSyncProviderMode,
    provider_status: ParentOwnedSyncProviderStatus,
    destination_ownership: ParentOwnedSyncExportDestinationOwnership,
    account_ref: Option<&'a str>,
    folder_ref: Option<&'a str>,
    revocation_ref: Option<&'a str>,
    disconnect_visibility_state: ParentOwnedSyncDisconnectVisibilityState,
    delete_visibility_state: ParentOwnedSyncDeleteVisibilityState,
    timestamp: &'a ParentTimestamp,
}

fn provider_status_row(
    input: &ParentOwnedSyncProviderStatusInput<'_>,
) -> ParentOwnedSyncProviderStatusRow {
    ParentOwnedSyncProviderStatusRow {
        provider_id: provider_id(format!("provider-{}", input.provider_mode.as_str())),
        provider_mode: input.provider_mode,
        provider_status: input.provider_status,
        destination_ownership: input.destination_ownership,
        account_ref: input.account_ref.map(provider_ref),
        folder_ref: input.folder_ref.map(provider_ref),
        status_ref: status_ref(format!(
            "provider-status-{}-{}",
            input.provider_mode.as_str(),
            input.provider_status.as_str()
        )),
        revocation_ref: input.revocation_ref.map(provider_ref),
        disconnect_visibility_state: input.disconnect_visibility_state,
        delete_visibility_state: input.delete_visibility_state,
        last_checked_at: input.timestamp.clone(),
        oauth_runtime_claimed: false,
        upload_runtime_claimed: false,
        delete_runtime_claimed: false,
        ocentra_hosted_family_data_stored: false,
        claim_safe: true,
    }
}
