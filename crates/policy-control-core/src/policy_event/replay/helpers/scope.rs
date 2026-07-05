#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;

use crate::policy_delivery::PolicyDeliveryId;
use crate::policy_event::{PolicyEventKind, PolicyEventScope};
use crate::policy_request::{PolicyApprovalId, PolicyOverrideId, PolicyRequestId};
use crate::policy_source::{
    ParentPolicyDocumentId, PolicyChildProfileId, PolicyConsumerDomain, PolicyDeviceId,
    PolicyHouseholdId, PolicyRollbackRef, PolicyVersion,
};

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
