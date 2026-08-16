#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{AggregateKey, EventType, IdempotencyKey};
use ocentra_parent_agent_protocol::constants::policy_control;

use crate::policy_event::{
    PolicyEvent, PolicyEventApplyOutcome, PolicyEventReplayRecord, PolicyEventSequence,
};

pub(crate) fn apply_policy_event_replay(
    current: &PolicyEventReplayRecord,
    next: &PolicyEvent,
) -> Result<PolicyEventApplyOutcome, EventingError> {
    let next_aggregate_key = super::scope_key::policy_event_scope_aggregate_key(next.scope())?;
    let next_idempotency_key = super::event_contract::policy_event_idempotency_key(next)?;

    assert_matching_aggregate_key(current, &next_aggregate_key)?;

    match next.sequence.value().cmp(&current.last_sequence.value()) {
        std::cmp::Ordering::Less => Ok(PolicyEventApplyOutcome::Stale(current.clone())),
        std::cmp::Ordering::Equal => {
            apply_equal_sequence_replay(current, next, &next_idempotency_key)
        }
        std::cmp::Ordering::Greater => Ok(PolicyEventApplyOutcome::Advanced(
            advanced_replay_record(next, next_aggregate_key, next_idempotency_key)?,
        )),
    }
}

fn conflicting_replay_value(sequence: PolicyEventSequence, last_event_type: &EventType) -> String {
    let mut value =
        String::from(policy_control::delivery::VALUE_CONFLICTING_REPLAY_FOR_SEQUENCE_PREFIX);
    value.push_str(&sequence.value().to_string());
    value.push_str(policy_control::delivery::VALUE_CONFLICTING_REPLAY_ON_SEPARATOR);
    value.push_str(last_event_type.as_str());
    value
}

fn advanced_replay_record(
    next: &PolicyEvent,
    aggregate_key: AggregateKey,
    idempotency_key: IdempotencyKey,
) -> Result<PolicyEventReplayRecord, EventingError> {
    Ok(PolicyEventReplayRecord {
        aggregate_key,
        last_sequence: next.sequence,
        last_event_type: super::event_contract::policy_event_event_type(next)?,
        last_idempotency_key: idempotency_key,
    })
}

fn apply_equal_sequence_replay(
    current: &PolicyEventReplayRecord,
    next: &PolicyEvent,
    next_idempotency_key: &IdempotencyKey,
) -> Result<PolicyEventApplyOutcome, EventingError> {
    if next_idempotency_key == &current.last_idempotency_key
        && super::event_contract::policy_event_event_type(next)? == current.last_event_type
    {
        return Ok(PolicyEventApplyOutcome::Duplicate(current.clone()));
    }

    Err(EventingError::InvalidValue {
        field: policy_control::delivery::FIELD_SEQUENCE,
        value: conflicting_replay_value(next.sequence, &current.last_event_type),
    })
}

fn assert_matching_aggregate_key(
    current: &PolicyEventReplayRecord,
    next_aggregate_key: &AggregateKey,
) -> Result<(), EventingError> {
    if next_aggregate_key != &current.aggregate_key {
        return Err(EventingError::InvalidValue {
            field: policy_control::source::FIELD_HOUSEHOLD_ID,
            value: next_aggregate_key.as_str().to_string(),
        });
    }

    Ok(())
}
