#![forbid(unsafe_code)]

//! Storage custody and retention ownership.
//!
//! This crate owns generic custody/delete/export decisions. Evidence crates own
//! evidence identity; feature crates own feature-specific interpretation.

pub const CRATE_NAME: &str = "ocentra-storage-custody-core";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageCustodyLocation {
    ChildDeviceLocal,
    ParentDeviceLocal,
    ParentOwnedRemote,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetentionWindowState {
    Active,
    Expired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParentExportState {
    Requested,
    NotRequested,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemoteSyncState {
    Enabled,
    Disabled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalPayloadRetentionAction {
    Delete,
    Retain,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParentExportPacketState {
    Create,
    DoNotCreate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemoteUploadState {
    Allowed,
    Blocked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StorageCustodyInput {
    pub location: StorageCustodyLocation,
    pub retention_window_state: RetentionWindowState,
    pub parent_export_state: ParentExportState,
    pub remote_sync_state: RemoteSyncState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StorageCustodyDecision {
    pub local_payload_retention_action: LocalPayloadRetentionAction,
    pub parent_export_packet_state: ParentExportPacketState,
    pub remote_upload_state: RemoteUploadState,
}

pub fn evaluate_storage_custody(input: StorageCustodyInput) -> StorageCustodyDecision {
    let delete_local_payload = input.retention_window_state == RetentionWindowState::Expired;
    let create_parent_export_packet = input.parent_export_state == ParentExportState::Requested;
    let remote_upload_allowed = input.remote_sync_state == RemoteSyncState::Enabled
        && input.location == StorageCustodyLocation::ParentOwnedRemote;

    StorageCustodyDecision {
        local_payload_retention_action: if delete_local_payload {
            LocalPayloadRetentionAction::Delete
        } else {
            LocalPayloadRetentionAction::Retain
        },
        parent_export_packet_state: if create_parent_export_packet {
            ParentExportPacketState::Create
        } else {
            ParentExportPacketState::DoNotCreate
        },
        remote_upload_state: if remote_upload_allowed {
            RemoteUploadState::Allowed
        } else {
            RemoteUploadState::Blocked
        },
    }
}
