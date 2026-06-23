use ocentra_eventing::{envelope::DomainEvent, error::EventingError};
use ocentra_storage_custody_core::storage_custody::{
    evaluate_storage_custody, plan_storage_custody_actions, storage_custody_action_planned_event,
    storage_custody_decision_recorded_event, LocalPayloadRetentionAction, ParentExportPacketState,
    ParentExportState, RemoteSyncState, RemoteUploadState, RetentionWindowState, StorageAuditState,
    StorageCustodyAggregateId, StorageCustodyDecisionId, StorageCustodyInput,
    StorageCustodyLocation, StorageTombstoneState,
};

const STORAGE_CUSTODY_AGGREGATE_ID: &str = "storage-custody-family-default";
const STORAGE_CUSTODY_DECISION_ID: &str = "storage-custody-decision-default";
const STORAGE_CUSTODY_DECISION_EVENT_TYPE: &str = "storage-custody.decision.recorded";
const STORAGE_CUSTODY_ACTION_EVENT_TYPE: &str = "storage-custody.action.planned";

#[test]
fn expired_parent_remote_payload_deletes_exports_uploads_and_tombstones() {
    let input = StorageCustodyInput {
        location: StorageCustodyLocation::ParentOwnedRemote,
        retention_window_state: RetentionWindowState::Expired,
        parent_export_state: ParentExportState::Requested,
        remote_sync_state: RemoteSyncState::Enabled,
    };

    let decision = evaluate_storage_custody(input);
    let actions = plan_storage_custody_actions(input);

    assert_eq!(
        decision.local_payload_retention_action,
        LocalPayloadRetentionAction::Delete
    );
    assert_eq!(
        decision.parent_export_packet_state,
        ParentExportPacketState::Create
    );
    assert_eq!(decision.remote_upload_state, RemoteUploadState::Allowed);
    assert_eq!(actions.tombstone_state, StorageTombstoneState::Write);
    assert_eq!(actions.audit_state, StorageAuditState::Record);
}

#[test]
fn local_child_payload_never_remote_uploads_even_when_sync_enabled() {
    let input = StorageCustodyInput {
        location: StorageCustodyLocation::ChildDeviceLocal,
        retention_window_state: RetentionWindowState::Active,
        parent_export_state: ParentExportState::NotRequested,
        remote_sync_state: RemoteSyncState::Enabled,
    };

    let decision = evaluate_storage_custody(input);
    let actions = plan_storage_custody_actions(input);

    assert_eq!(
        decision.local_payload_retention_action,
        LocalPayloadRetentionAction::Retain
    );
    assert_eq!(decision.remote_upload_state, RemoteUploadState::Blocked);
    assert_eq!(actions.tombstone_state, StorageTombstoneState::DoNotWrite);
}

#[test]
fn custody_decision_event_projects_typed_action_event() -> Result<(), EventingError> {
    let decision_event = storage_custody_decision_recorded_event(
        StorageCustodyAggregateId::parse(STORAGE_CUSTODY_AGGREGATE_ID)?,
        StorageCustodyDecisionId::parse(STORAGE_CUSTODY_DECISION_ID)?,
        StorageCustodyInput {
            location: StorageCustodyLocation::ParentDeviceLocal,
            retention_window_state: RetentionWindowState::Expired,
            parent_export_state: ParentExportState::Requested,
            remote_sync_state: RemoteSyncState::Disabled,
        },
    );

    let action_event = storage_custody_action_planned_event(decision_event.clone());

    assert_eq!(
        decision_event.contract()?.event_type.as_str(),
        STORAGE_CUSTODY_DECISION_EVENT_TYPE
    );
    assert_eq!(
        action_event.contract()?.event_type.as_str(),
        STORAGE_CUSTODY_ACTION_EVENT_TYPE
    );
    assert_eq!(action_event.aggregate_id, decision_event.aggregate_id);
    assert_eq!(action_event.source_decision_id, decision_event.decision_id);

    Ok(())
}
