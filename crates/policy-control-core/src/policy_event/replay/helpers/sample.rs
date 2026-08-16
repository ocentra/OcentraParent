#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;

use crate::policy_event::{
    PolicyEvent, PolicyEventDeadLetterReason, PolicyEventKind, PolicyEventSequence,
};
use crate::policy_source::{PolicyAuditReferenceId, PolicyReasonCode};

pub(crate) fn sample_policy_event(kind: PolicyEventKind) -> Result<PolicyEvent, EventingError> {
    let scope = super::scope::sample_policy_event_scope(kind)?;
    let audit_reference_ids = vec![PolicyAuditReferenceId::parse("audit-policy-event")?];
    let reason_code = if kind_requires_reason(kind) {
        Some(PolicyReasonCode::parse(kind.reason_code_value())?)
    } else {
        None
    };
    let dead_letter_reason = if matches!(kind, PolicyEventKind::DeadLetterRecorded) {
        Some(PolicyEventDeadLetterReason::ReplayRejected)
    } else {
        None
    };
    Ok(PolicyEvent {
        schema_version: super::policy_event_schema_version()?,
        kind,
        sequence: PolicyEventSequence::new(1)?,
        scope,
        audit_reference_ids,
        reason_code,
        dead_letter_reason,
    })
}

pub(crate) fn kind_requires_reason(kind: PolicyEventKind) -> bool {
    matches!(
        kind,
        PolicyEventKind::DeliveryRejected
            | PolicyEventKind::DeliveryExpired
            | PolicyEventKind::DeliveryRetryScheduled
            | PolicyEventKind::DomainPartial
            | PolicyEventKind::AskParentDenied
            | PolicyEventKind::OverrideExpired
            | PolicyEventKind::ManualRequired
            | PolicyEventKind::RollbackApplied
    )
}

pub(crate) fn policy_event_kind_reason_code_value(kind: PolicyEventKind) -> &'static str {
    match kind {
        PolicyEventKind::DeliveryRejected => "delivery-rejected",
        PolicyEventKind::DeliveryExpired => "delivery-expired",
        PolicyEventKind::DeliveryRetryScheduled => "delivery-retry-scheduled",
        PolicyEventKind::DomainPartial => "domain-partial",
        PolicyEventKind::AskParentDenied => "ask-parent-denied",
        PolicyEventKind::OverrideExpired => "override-expired",
        PolicyEventKind::ManualRequired => "manual-required",
        PolicyEventKind::RollbackApplied => "rollback-applied",
        _ => "policy-event",
    }
}
