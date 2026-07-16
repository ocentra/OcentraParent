#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;

use crate::policy_event::{PolicyEvent, PolicyEventDeadLetterReason};
use crate::policy_source::{PolicyAuditReferenceId, PolicyReasonCode};

pub(crate) fn policy_event_idempotency_key_value(
    event: &PolicyEvent,
) -> Result<String, EventingError> {
    let aggregate_key = super::scope_key::policy_event_scope_aggregate_key(event.scope())?;
    let mut value = String::from("policy-event:");
    value.push_str(event.kind.event_type_name());
    value.push('|');
    value.push_str(aggregate_key.as_str());
    value.push('|');
    value.push_str(&event.sequence.value().to_string());
    value.push('|');
    value.push_str(super::scope_label::policy_event_scope_family_label(
        event.scope(),
    ));
    value.push('|');
    value.push_str(&join_audit_reference_ids(&event.audit_reference_ids));
    value.push('|');
    value.push_str(
        event
            .reason_code
            .as_ref()
            .map_or("none", PolicyReasonCode::as_str),
    );
    value.push('|');
    value.push_str(
        event
            .dead_letter_reason
            .as_ref()
            .map_or("none", policy_event_dead_letter_reason_name),
    );
    Ok(value)
}

fn join_audit_reference_ids(audit_reference_ids: &[PolicyAuditReferenceId]) -> String {
    audit_reference_ids
        .iter()
        .map(PolicyAuditReferenceId::as_str)
        .collect::<Vec<_>>()
        .join(",")
}

fn policy_event_dead_letter_reason_name(reason: &PolicyEventDeadLetterReason) -> &'static str {
    match reason {
        PolicyEventDeadLetterReason::DuplicateIdempotency => "duplicate-idempotency",
        PolicyEventDeadLetterReason::ReplayRejected => "replay-rejected",
        PolicyEventDeadLetterReason::StaleSequence => "stale-sequence",
        PolicyEventDeadLetterReason::UnsupportedTarget => "unsupported-target",
        PolicyEventDeadLetterReason::MissingSubscriber => "missing-subscriber",
        PolicyEventDeadLetterReason::ManualRequired => "manual-required",
    }
}
