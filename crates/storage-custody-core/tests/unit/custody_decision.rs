use ocentra_eventing::{envelope::DomainEvent, error::EventingError};
use ocentra_storage_custody_core::storage_custody::{
    evaluate_storage_custody, plan_storage_custody_actions, storage_custody_action_planned_event,
    storage_custody_decision_recorded_event, LocalPayloadRetentionAction, ParentExportPacketState,
    ParentExportState, RemoteSyncState, RemoteUploadState, RetentionWindowState,
    StorageCustodyAggregateId, StorageCustodyDecisionId, StorageCustodyInput,
    StorageCustodyLocation, StorageTombstoneState,
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

#[test]
fn expired_payload_action_plan_writes_tombstone() {
    let plan = plan_storage_custody_actions(StorageCustodyInput {
        location: StorageCustodyLocation::ChildDeviceLocal,
        retention_window_state: RetentionWindowState::Expired,
        parent_export_state: ParentExportState::NotRequested,
        remote_sync_state: RemoteSyncState::Disabled,
    });

    assert_eq!(
        plan.local_payload_retention_action,
        LocalPayloadRetentionAction::Delete
    );
    assert_eq!(plan.tombstone_state, StorageTombstoneState::Write);
    assert_eq!(
        plan.parent_export_packet_state,
        ParentExportPacketState::DoNotCreate
    );
}

#[test]
fn active_parent_owned_remote_action_plan_allows_upload_without_tombstone() {
    let plan = plan_storage_custody_actions(StorageCustodyInput {
        location: StorageCustodyLocation::ParentOwnedRemote,
        retention_window_state: RetentionWindowState::Active,
        parent_export_state: ParentExportState::NotRequested,
        remote_sync_state: RemoteSyncState::Enabled,
    });

    assert_eq!(
        plan.local_payload_retention_action,
        LocalPayloadRetentionAction::Retain
    );
    assert_eq!(plan.tombstone_state, StorageTombstoneState::DoNotWrite);
    assert_eq!(plan.remote_upload_state, RemoteUploadState::Allowed);
}

#[test]
fn storage_custody_decision_event_drives_action_plan_event() -> Result<(), EventingError> {
    let decision = storage_custody_decision_recorded_event(
        StorageCustodyAggregateId::parse("storage-custody-child-default")?,
        StorageCustodyDecisionId::parse("storage-custody-decision-default")?,
        StorageCustodyInput {
            location: StorageCustodyLocation::ChildDeviceLocal,
            retention_window_state: RetentionWindowState::Expired,
            parent_export_state: ParentExportState::Requested,
            remote_sync_state: RemoteSyncState::Disabled,
        },
    );
    let action = storage_custody_action_planned_event(decision.clone());

    assert_eq!(action.aggregate_id, decision.aggregate_id);
    assert_eq!(action.source_decision_id, decision.decision_id);
    assert_eq!(
        action.action_plan.tombstone_state,
        StorageTombstoneState::Write
    );
    assert_eq!(
        action.action_plan.parent_export_packet_state,
        ParentExportPacketState::Create
    );
    assert_eq!(
        decision.contract()?.event_type.as_str(),
        "storage-custody.decision.recorded"
    );
    assert_eq!(
        action.contract()?.event_type.as_str(),
        "storage-custody.action.planned"
    );

    Ok(())
}
