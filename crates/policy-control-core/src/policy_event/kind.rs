#![forbid(unsafe_code)]

use super::PolicyEventKind;

const POLICY_EVENT_KIND_NAMES: [&str; 25] = [
    "policy.draft.created",
    "policy.preview.requested",
    "policy.preview.generated",
    "policy.confirmed",
    "policy.version.superseded",
    "policy.compiler.requested",
    "policy.compiler.completed",
    "policy.delivery.queued",
    "policy.delivery.sent",
    "policy.delivery.acknowledged",
    "policy.delivery.rejected",
    "policy.delivery.expired",
    "policy.delivery.retry-scheduled",
    "policy.domain.applied",
    "policy.domain.partial",
    "policy.rollback.requested",
    "policy.rollback.applied",
    "policy.ask-parent.requested",
    "policy.ask-parent.approved",
    "policy.ask-parent.denied",
    "policy.override.created",
    "policy.override.expired",
    "policy.audit.recorded",
    "policy.dead-letter.recorded",
    "policy.manual-required",
];

pub(crate) const POLICY_EVENT_KINDS: &[PolicyEventKind] = &[
    PolicyEventKind::DraftCreated,
    PolicyEventKind::PreviewRequested,
    PolicyEventKind::PreviewGenerated,
    PolicyEventKind::Confirmed,
    PolicyEventKind::VersionSuperseded,
    PolicyEventKind::CompilerRequested,
    PolicyEventKind::CompilerCompleted,
    PolicyEventKind::DeliveryQueued,
    PolicyEventKind::DeliverySent,
    PolicyEventKind::DeliveryAcknowledged,
    PolicyEventKind::DeliveryRejected,
    PolicyEventKind::DeliveryExpired,
    PolicyEventKind::DeliveryRetryScheduled,
    PolicyEventKind::DomainApplied,
    PolicyEventKind::DomainPartial,
    PolicyEventKind::RollbackRequested,
    PolicyEventKind::RollbackApplied,
    PolicyEventKind::AskParentRequested,
    PolicyEventKind::AskParentApproved,
    PolicyEventKind::AskParentDenied,
    PolicyEventKind::OverrideCreated,
    PolicyEventKind::OverrideExpired,
    PolicyEventKind::AuditRecorded,
    PolicyEventKind::DeadLetterRecorded,
    PolicyEventKind::ManualRequired,
];

pub(crate) fn policy_event_kind_name(kind: PolicyEventKind) -> &'static str {
    POLICY_EVENT_KIND_NAMES[kind as usize]
}

pub(crate) fn policy_event_kinds() -> &'static [PolicyEventKind] {
    POLICY_EVENT_KINDS
}
