#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;

use crate::policy_event::{PolicyEvent, PolicyEventKind, PolicyEventScope};

const POLICY_EVENT_SCOPE_FIELD: &str = "policy_event.scope";
const SCOPE_MISMATCH_PREFIX: &str = "scope does not match event kind: expected ";
const SCOPE_MISMATCH_SEPARATOR: &str = ", received ";
const ROLLBACK_HOUSEHOLD_MISMATCH: &str = "rollback household mismatch";

pub(super) fn validate(event: &PolicyEvent) -> Result<(), EventingError> {
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

    validate_scope_identity(event)
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
