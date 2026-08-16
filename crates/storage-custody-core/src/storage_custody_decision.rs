use super::{
    LocalPayloadRetentionAction, ParentExportPacketState, ParentExportState, RemoteSyncState,
    RemoteUploadState, RetentionWindowState, StorageCustodyDecision, StorageCustodyInput,
};

pub(super) fn evaluate_storage_custody(input: StorageCustodyInput) -> StorageCustodyDecision {
    let delete_local_payload = input.retention_window_state == RetentionWindowState::Expired;
    let create_parent_export_packet = input.parent_export_state == ParentExportState::Requested;
    let remote_upload_allowed = input.remote_sync_state == RemoteSyncState::Enabled
        && input.location == super::StorageCustodyLocation::ParentOwnedRemote;

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
