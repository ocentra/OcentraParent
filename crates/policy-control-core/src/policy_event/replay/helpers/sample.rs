#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;

use crate::policy_delivery::PolicyDeliveryId;
use crate::policy_event::{
    PolicyEvent, PolicyEventDeadLetterReason, PolicyEventKind, PolicyEventScope,
    PolicyEventSequence,
};
use crate::policy_request::{PolicyApprovalId, PolicyOverrideId, PolicyRequestId};
use crate::policy_source::{
    ParentPolicyDocumentId, PolicyAuditReferenceId, PolicyChildProfileId, PolicyConsumerDomain,
    PolicyDeviceId, PolicyHouseholdId, PolicyReasonCode, PolicyRollbackRef, PolicyVersion,
};

pub(crate) fn sample_policy_event(kind: PolicyEventKind) -> Result<PolicyEvent, EventingError> {
    let scope = sample_policy_event_scope(kind)?;
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

pub(crate) fn sample_policy_event_scope(
    kind: PolicyEventKind,
) -> Result<PolicyEventScope, EventingError> {
    let household_id = PolicyHouseholdId::parse("household-default")?;
    let source_document_id = ParentPolicyDocumentId::parse("policy-source-default")?;
    let policy_version = PolicyVersion::new(5)?;

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
        | PolicyEventKind::ManualRequired => Ok(PolicyEventScope::SourceDocument {
            household_id,
            source_document_id,
            policy_version,
        }),
        PolicyEventKind::AskParentRequested
        | PolicyEventKind::AskParentApproved
        | PolicyEventKind::AskParentDenied => Ok(PolicyEventScope::Request {
            household_id,
            request_id: PolicyRequestId::parse("policy-request-default")?,
            child_profile_id: PolicyChildProfileId::parse("child-primary")?,
            source_document_id,
            policy_version,
        }),
        PolicyEventKind::OverrideCreated | PolicyEventKind::OverrideExpired => {
            Ok(PolicyEventScope::Override {
                household_id,
                override_id: PolicyOverrideId::parse("policy-override-default")?,
                approval_id: PolicyApprovalId::parse("policy-approval-default")?,
                request_id: PolicyRequestId::parse("policy-request-default")?,
                source_document_id,
                policy_version,
            })
        }
        PolicyEventKind::DeliveryQueued
        | PolicyEventKind::DeliverySent
        | PolicyEventKind::DeliveryAcknowledged
        | PolicyEventKind::DeliveryRejected
        | PolicyEventKind::DeliveryExpired
        | PolicyEventKind::DeliveryRetryScheduled
        | PolicyEventKind::DomainApplied
        | PolicyEventKind::DomainPartial => Ok(PolicyEventScope::Delivery {
            household_id,
            delivery_id: PolicyDeliveryId::parse("policy-delivery-default")?,
            child_profile_id: PolicyChildProfileId::parse("child-primary")?,
            device_id: PolicyDeviceId::parse("device-laptop")?,
            domain: PolicyConsumerDomain::Tracking,
            source_document_id,
            policy_version,
        }),
        PolicyEventKind::RollbackRequested | PolicyEventKind::RollbackApplied => {
            Ok(PolicyEventScope::Rollback {
                household_id,
                rollback_ref: PolicyRollbackRef {
                    household_id: PolicyHouseholdId::parse("household-default")?,
                    rolled_back_document_id: ParentPolicyDocumentId::parse(
                        "policy-source-default",
                    )?,
                    rolled_back_policy_version: PolicyVersion::new(5)?,
                    restored_document_id: ParentPolicyDocumentId::parse("policy-source-previous")?,
                    restored_policy_version: PolicyVersion::new(4)?,
                },
            })
        }
    }
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
