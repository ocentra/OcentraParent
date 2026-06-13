use ocentra_storage_custody_core::{
    evaluate_storage_custody, LocalPayloadRetentionAction, ParentExportPacketState,
    ParentExportState, RemoteSyncState, RemoteUploadState, RetentionWindowState,
    StorageCustodyInput, StorageCustodyLocation,
};

#[test]
fn expired_retention_deletes_local_payload_without_remote_upload() {
    let decision = evaluate_storage_custody(StorageCustodyInput {
        location: StorageCustodyLocation::ChildDeviceLocal,
        retention_window_state: RetentionWindowState::Expired,
        parent_export_state: ParentExportState::NotRequested,
        remote_sync_state: RemoteSyncState::Disabled,
    });

    assert_eq!(
        decision.local_payload_retention_action,
        LocalPayloadRetentionAction::Delete
    );
    assert_eq!(
        decision.parent_export_packet_state,
        ParentExportPacketState::DoNotCreate
    );
    assert_eq!(decision.remote_upload_state, RemoteUploadState::Blocked);
}

#[test]
fn parent_export_packet_is_separate_from_remote_sync() {
    let decision = evaluate_storage_custody(StorageCustodyInput {
        location: StorageCustodyLocation::ParentDeviceLocal,
        retention_window_state: RetentionWindowState::Active,
        parent_export_state: ParentExportState::Requested,
        remote_sync_state: RemoteSyncState::Disabled,
    });

    assert_eq!(
        decision.local_payload_retention_action,
        LocalPayloadRetentionAction::Retain
    );
    assert_eq!(
        decision.parent_export_packet_state,
        ParentExportPacketState::Create
    );
    assert_eq!(decision.remote_upload_state, RemoteUploadState::Blocked);
}

#[test]
fn remote_upload_requires_parent_owned_remote_custody() {
    let decision = evaluate_storage_custody(StorageCustodyInput {
        location: StorageCustodyLocation::ParentOwnedRemote,
        retention_window_state: RetentionWindowState::Active,
        parent_export_state: ParentExportState::NotRequested,
        remote_sync_state: RemoteSyncState::Enabled,
    });

    assert_eq!(decision.remote_upload_state, RemoteUploadState::Allowed);
}

#[test]
fn expired_retention_can_delete_local_payload_and_still_create_parent_export_packet() {
    let decision = evaluate_storage_custody(StorageCustodyInput {
        location: StorageCustodyLocation::ChildDeviceLocal,
        retention_window_state: RetentionWindowState::Expired,
        parent_export_state: ParentExportState::Requested,
        remote_sync_state: RemoteSyncState::Enabled,
    });

    assert_eq!(
        decision.local_payload_retention_action,
        LocalPayloadRetentionAction::Delete
    );
    assert_eq!(
        decision.parent_export_packet_state,
        ParentExportPacketState::Create
    );
    assert_eq!(decision.remote_upload_state, RemoteUploadState::Blocked);
}
