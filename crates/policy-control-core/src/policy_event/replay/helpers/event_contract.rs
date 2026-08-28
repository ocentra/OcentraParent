#![forbid(unsafe_code)]

use std::collections::BTreeSet;

use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{AggregateKey, EventType, IdempotencyKey};

use crate::policy_event::{
    PolicyEvent, PolicyEventKind, PolicyEventReplayRecord, PolicyEventScope,
};

const POLICY_EVENT_SCHEMA_VERSION_FIELD: &str = "policy_event.schema_version";
const POLICY_EVENT_SCOPE_FIELD: &str = "policy_event.scope";
const POLICY_EVENT_AUDIT_REFERENCES_FIELD: &str = "policy_event.audit_reference_ids";
const POLICY_EVENT_REASON_CODE_FIELD: &str = "policy_event.reason_code";
const POLICY_EVENT_DEAD_LETTER_REASON_FIELD: &str = "policy_event.dead_letter_reason";
const MISSING_AUDIT_REFERENCES: &str = "missing audit references";
const DUPLICATE_AUDIT_REFERENCE: &str = "duplicate audit reference";
const UNSUPPORTED_SCHEMA_VERSION_PREFIX: &str = "unsupported schema version ";
const SCOPE_MISMATCH_PREFIX: &str = "scope does not match event kind: expected ";
const SCOPE_MISMATCH_SEPARATOR: &str = ", received ";
const ROLLBACK_HOUSEHOLD_MISMATCH: &str = "rollback household mismatch";
const MISSING_REASON_PREFIX: &str = "missing reason code for ";
const UNEXPECTED_REASON_PREFIX: &str = "unexpected reason code for ";
const INVALID_REASON_PREFIX: &str = "invalid reason code for ";
const DEAD_LETTER_REASON_REQUIRED: &str = "dead-letter reason required";
const DEAD_LETTER_REASON_UNEXPECTED: &str =
    "dead-letter reason only valid for policy.dead-letter.recorded";

pub(crate) fn validate_policy_event(event: &PolicyEvent) -> Result<(), EventingError> {
    let expected_schema_version = super::registry::policy_event_schema_version()?;
    if event.schema_version != expected_schema_version {
        return Err(EventingError::InvalidValue {
            field: POLICY_EVENT_SCHEMA_VERSION_FIELD,
            value: format!(
                "{UNSUPPORTED_SCHEMA_VERSION_PREFIX}{}",
                event.schema_version.value()
            ),
        });
    }

    let expected_scope = expected_scope_family(event.kind);
    let received_scope = event.scope.family_label();
    if expected_scope != received_scope {
        return Err(EventingError::InvalidValue {
            field: POLICY_EVENT_SCOPE_FIELD,
            value: format!(
                "{SCOPE_MISMATCH_PREFIX}{expected_scope}{SCOPE_MISMATCH_SEPARATOR}{received_scope}"
            ),
        });
    }

    validate_scope_identity(event)?;
    validate_audit_references(event)?;
    validate_reason_code(event)?;
    validate_dead_letter_reason(event)
}

fn validate_scope_identity(event: &PolicyEvent) -> Result<(), EventingError> {
    if let PolicyEventScope::Rollback {
        household_id,
        rollback_ref,
    } = &event.scope
    {
        if household_id != &rollback_ref.household_id {
            return Err(EventingError::InvalidValue {
                field: POLICY_EVENT_SCOPE_FIELD,
                value: ROLLBACK_HOUSEHOLD_MISMATCH.to_string(),
            });
        }
    }

    Ok(())
}

fn validate_audit_references(event: &PolicyEvent) -> Result<(), EventingError> {
    if event.audit_reference_ids.is_empty() {
        return Err(EventingError::InvalidValue {
            field: POLICY_EVENT_AUDIT_REFERENCES_FIELD,
            value: MISSING_AUDIT_REFERENCES.to_string(),
        });
    }

    let mut seen = BTreeSet::new();
    for audit_reference_id in &event.audit_reference_ids {
        if !seen.insert(audit_reference_id) {
            return Err(EventingError::InvalidValue {
                field: POLICY_EVENT_AUDIT_REFERENCES_FIELD,
                value: DUPLICATE_AUDIT_REFERENCE.to_string(),
            });
        }
    }

    Ok(())
}

fn validate_reason_code(event: &PolicyEvent) -> Result<(), EventingError> {
    let expected_reason = event.kind.reason_code_value();
    if super::sample::kind_requires_reason(event.kind) {
        match event.reason_code.as_ref() {
            Some(reason_code) if reason_code.as_str() == expected_reason => Ok(()),
            Some(_) => Err(EventingError::InvalidValue {
                field: POLICY_EVENT_REASON_CODE_FIELD,
                value: format!("{INVALID_REASON_PREFIX}{expected_reason}"),
            }),
            None => Err(EventingError::InvalidValue {
                field: POLICY_EVENT_REASON_CODE_FIELD,
                value: format!("{MISSING_REASON_PREFIX}{expected_reason}"),
            }),
        }
    } else if event.reason_code.is_some() {
        Err(EventingError::InvalidValue {
            field: POLICY_EVENT_REASON_CODE_FIELD,
            value: format!("{UNEXPECTED_REASON_PREFIX}{}", event.kind.event_type_name()),
        })
    } else {
        Ok(())
    }
}

fn validate_dead_letter_reason(event: &PolicyEvent) -> Result<(), EventingError> {
    if matches!(event.kind, PolicyEventKind::DeadLetterRecorded) {
        if event.dead_letter_reason.is_none() {
            return Err(EventingError::InvalidValue {
                field: POLICY_EVENT_DEAD_LETTER_REASON_FIELD,
                value: DEAD_LETTER_REASON_REQUIRED.to_string(),
            });
        }
    } else if event.dead_letter_reason.is_some() {
        return Err(EventingError::InvalidValue {
            field: POLICY_EVENT_DEAD_LETTER_REASON_FIELD,
            value: DEAD_LETTER_REASON_UNEXPECTED.to_string(),
        });
    }

    Ok(())
}

fn expected_scope_family(kind: PolicyEventKind) -> &'static str {
    match kind {
        PolicyEventKind::DraftCreated
        | PolicyEventKind::PreviewRequested
        | PolicyEventKind::PreviewGenerated
        | PolicyEventKind::Confirmed
        | PolicyEventKind::VersionSuperseded
        | PolicyEventKind::CompilerRequested
        | PolicyEventKind::CompilerCompleted
        | PolicyEventKind::AuditRecorded
        | PolicyEventKind::DeadLetterRecorded
        | PolicyEventKind::ManualRequired => "source-document",
        PolicyEventKind::AskParentRequested
        | PolicyEventKind::AskParentApproved
        | PolicyEventKind::AskParentDenied => "request",
        PolicyEventKind::OverrideCreated | PolicyEventKind::OverrideExpired => "override",
        PolicyEventKind::DeliveryQueued
        | PolicyEventKind::DeliverySent
        | PolicyEventKind::DeliveryAcknowledged
        | PolicyEventKind::DeliveryRejected
        | PolicyEventKind::DeliveryExpired
        | PolicyEventKind::DeliveryRetryScheduled
        | PolicyEventKind::DomainApplied
        | PolicyEventKind::DomainPartial => "delivery",
        PolicyEventKind::RollbackRequested | PolicyEventKind::RollbackApplied => "rollback",
    }
}

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
    validate_policy_event(event)?;
    Ok(ocentra_eventing::envelope::EventContract::new(
        policy_event_event_type(event)?,
        event.schema_version,
    ))
}

pub(crate) fn policy_event_aggregate_key(
    event: &PolicyEvent,
) -> Result<AggregateKey, EventingError> {
    validate_policy_event(event)?;
    super::scope_key::policy_event_scope_aggregate_key(&event.scope)
}

pub(crate) fn policy_event_idempotency_key(
    event: &PolicyEvent,
) -> Result<IdempotencyKey, EventingError> {
    validate_policy_event(event)?;
    IdempotencyKey::parse(super::idempotency::policy_event_idempotency_key_value(
        event,
    )?)
}

pub(crate) fn policy_event_replay_record(
    event: &PolicyEvent,
) -> Result<PolicyEventReplayRecord, EventingError> {
    validate_policy_event(event)?;
    Ok(PolicyEventReplayRecord {
        aggregate_key: policy_event_aggregate_key(event)?,
        last_sequence: event.sequence,
        last_event_type: policy_event_event_type(event)?,
        last_idempotency_key: policy_event_idempotency_key(event)?,
    })
}
