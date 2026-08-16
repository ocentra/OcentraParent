#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{AggregateKey, EventType, IdempotencyKey};

use crate::policy_event::{PolicyEvent, PolicyEventReplayRecord};

pub(crate) fn policy_event_redacted_summary(event: &PolicyEvent) -> String {
    let mut value = String::from("policy-event kind=");
    value.push_str(event.kind.event_type_name());
    value.push_str(" scope=");
    value.push_str(super::scope_label::policy_event_scope_family_label(
        &event.scope,
    ));
    value.push_str(" sequence=");
    value.push_str(&event.sequence.value().to_string());
    if matches!(
        event.kind,
        crate::policy_event::PolicyEventKind::ManualRequired
    ) {
        value.push_str(" manual-required");
    }
    if matches!(
        event.kind,
        crate::policy_event::PolicyEventKind::DeadLetterRecorded
    ) {
        value.push_str(" dead-lettered");
    }
    value
}

pub(crate) fn policy_event_event_type(event: &PolicyEvent) -> Result<EventType, EventingError> {
    EventType::parse(event.kind.event_type_name())
}

pub(crate) fn policy_event_contract(
    event: &PolicyEvent,
) -> Result<ocentra_eventing::envelope::EventContract, EventingError> {
    Ok(ocentra_eventing::envelope::EventContract::new(
        policy_event_event_type(event)?,
        event.schema_version,
    ))
}

pub(crate) fn policy_event_aggregate_key(
    event: &PolicyEvent,
) -> Result<AggregateKey, EventingError> {
    super::scope_key::policy_event_scope_aggregate_key(&event.scope)
}

pub(crate) fn policy_event_idempotency_key(
    event: &PolicyEvent,
) -> Result<IdempotencyKey, EventingError> {
    IdempotencyKey::parse(super::idempotency::policy_event_idempotency_key_value(
        event,
    )?)
}

pub(crate) fn policy_event_replay_record(
    event: &PolicyEvent,
) -> Result<PolicyEventReplayRecord, EventingError> {
    Ok(PolicyEventReplayRecord {
        aggregate_key: policy_event_aggregate_key(event)?,
        last_sequence: event.sequence,
        last_event_type: policy_event_event_type(event)?,
        last_idempotency_key: policy_event_idempotency_key(event)?,
    })
}
