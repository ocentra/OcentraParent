use ocentra_eventing::envelope::EventContract;
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{EventType, IdempotencyKey, SchemaVersion};

use super::{
    evaluate_storage_custody, StorageCustodyActionPlan, StorageCustodyActionPlanId,
    StorageCustodyActionPlannedEvent, StorageCustodyAggregateId, StorageCustodyDecisionId,
    StorageCustodyDecisionRecordedEvent, StorageCustodyInput, STORAGE_CUSTODY_ACTION_PREFIX,
    STORAGE_CUSTODY_IDEMPOTENCY_SEPARATOR, STORAGE_CUSTODY_SCHEMA_VERSION,
};

pub(super) fn storage_custody_event_contract(
    event_type: &str,
) -> Result<EventContract, EventingError> {
    Ok(EventContract::new(
        EventType::parse(event_type)?,
        SchemaVersion::new(STORAGE_CUSTODY_SCHEMA_VERSION)?,
    ))
}

pub(super) fn storage_custody_idempotency_key(
    event_type: &str,
    unique_ref: impl std::fmt::Display,
) -> Result<IdempotencyKey, EventingError> {
    IdempotencyKey::parse(format!(
        "{}{}{}",
        event_type, STORAGE_CUSTODY_IDEMPOTENCY_SEPARATOR, unique_ref
    ))
}

pub(super) fn storage_custody_decision_recorded_event(
    aggregate_id: StorageCustodyAggregateId,
    decision_id: StorageCustodyDecisionId,
    input: StorageCustodyInput,
) -> StorageCustodyDecisionRecordedEvent {
    StorageCustodyDecisionRecordedEvent {
        aggregate_id,
        decision_id,
        input,
        decision: evaluate_storage_custody(input),
    }
}

pub(super) fn plan_storage_custody_actions(input: StorageCustodyInput) -> StorageCustodyActionPlan {
    let decision = evaluate_storage_custody(input);
    StorageCustodyActionPlan {
        local_payload_retention_action: decision.local_payload_retention_action,
        tombstone_state: if decision.local_payload_retention_action
            == super::LocalPayloadRetentionAction::Delete
        {
            super::StorageTombstoneState::Write
        } else {
            super::StorageTombstoneState::DoNotWrite
        },
        parent_export_packet_state: decision.parent_export_packet_state,
        remote_upload_state: decision.remote_upload_state,
        audit_state: super::StorageAuditState::Record,
    }
}

pub(super) fn storage_custody_action_planned_event(
    event: StorageCustodyDecisionRecordedEvent,
) -> StorageCustodyActionPlannedEvent {
    StorageCustodyActionPlannedEvent {
        aggregate_id: event.aggregate_id,
        action_plan_id: StorageCustodyActionPlanId(storage_custody_action_ref(&event.decision_id)),
        source_decision_id: event.decision_id,
        action_plan: plan_storage_custody_actions(event.input),
    }
}

fn storage_custody_action_ref(decision_id: &StorageCustodyDecisionId) -> String {
    let mut value = String::from(STORAGE_CUSTODY_ACTION_PREFIX);
    value.push_str(decision_id.as_str());
    value
}
