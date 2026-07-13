use ocentra_eventing::envelope::DomainEvent;
use ocentra_storage_custody_core::storage_custody::{
    storage_custody_action_planned_event, storage_custody_decision_recorded_event,
    ParentExportState, RemoteSyncState, RetentionWindowState, StorageCustodyActionPlannedEvent,
    StorageCustodyAggregateId, StorageCustodyDecisionId, StorageCustodyInput,
    StorageCustodyLocation,
};

const STORAGE_CUSTODY_AGGREGATE_ID: &str = "storage-custody-contract-default";
const STORAGE_CUSTODY_DECISION_ID: &str = "storage-custody-decision-contract-default";

#[test]
fn storage_custody_decision_and_action_events_keep_contract_surface_stable() {
    let decision_event = storage_custody_decision_recorded_event(
        StorageCustodyAggregateId::parse(STORAGE_CUSTODY_AGGREGATE_ID).expect("aggregate id"),
        StorageCustodyDecisionId::parse(STORAGE_CUSTODY_DECISION_ID).expect("decision id"),
        StorageCustodyInput {
            location: StorageCustodyLocation::ParentOwnedRemote,
            retention_window_state: RetentionWindowState::Expired,
            parent_export_state: ParentExportState::Requested,
            remote_sync_state: RemoteSyncState::Enabled,
        },
    );
    let action_event = storage_custody_action_planned_event(decision_event.clone());

    assert_eq!(
        decision_event
            .contract()
            .expect("decision contract")
            .event_type
            .as_str(),
        "storage-custody.decision.recorded"
    );
    assert_eq!(
        action_event
            .contract()
            .expect("action contract")
            .event_type
            .as_str(),
        "storage-custody.action.planned"
    );

    let decision_json = serde_json::to_value(&decision_event).expect("serialize decision event");
    let action_json = serde_json::to_value(&action_event).expect("serialize action event");
    let round_trip_decision: StorageCustodyActionPlannedEvent =
        serde_json::from_value(action_json).expect("deserialize action event");
    let round_trip_request = serde_json::from_value::<
        ocentra_storage_custody_core::storage_custody::StorageCustodyDecisionRecordedEvent,
    >(decision_json)
    .expect("deserialize decision event");

    assert_eq!(round_trip_request, decision_event);
    assert_eq!(round_trip_decision, action_event);
}
