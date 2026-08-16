#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::AggregateKey;

use crate::policy_delivery::PolicyDeliveryId;
use crate::policy_event::PolicyEventScope;
use crate::policy_request::{PolicyApprovalId, PolicyOverrideId, PolicyRequestId};
use crate::policy_source::{
    ParentPolicyDocumentId, PolicyAuditReferenceId, PolicyChildProfileId, PolicyConsumerDomain,
    PolicyDeviceId, PolicyHouseholdId, PolicyRollbackRef, PolicyVersion,
};

pub(crate) fn policy_event_scope_aggregate_key(
    scope: &PolicyEventScope,
) -> Result<AggregateKey, EventingError> {
    AggregateKey::parse(policy_event_scope_aggregate_key_value(scope))
}

fn policy_event_scope_aggregate_key_value(scope: &PolicyEventScope) -> String {
    match scope {
        PolicyEventScope::SourceDocument {
            household_id,
            source_document_id,
            policy_version,
        } => source_document_aggregate_key_value(household_id, source_document_id, policy_version),
        PolicyEventScope::Request {
            household_id,
            request_id,
            policy_version,
            ..
        } => request_aggregate_key_value(household_id, request_id, policy_version),
        PolicyEventScope::Approval {
            household_id,
            approval_id,
            request_id,
            policy_version,
            ..
        } => approval_aggregate_key_value(household_id, approval_id, request_id, policy_version),
        PolicyEventScope::Override {
            household_id,
            override_id,
            approval_id,
            request_id,
            policy_version,
            ..
        } => override_aggregate_key_value(
            household_id,
            override_id,
            approval_id,
            request_id,
            policy_version,
        ),
        PolicyEventScope::Delivery {
            household_id,
            delivery_id,
            child_profile_id,
            device_id,
            domain,
            policy_version,
            ..
        } => delivery_aggregate_key_value(
            household_id,
            delivery_id,
            child_profile_id,
            device_id,
            *domain,
            policy_version,
        ),
        PolicyEventScope::Rollback {
            household_id,
            rollback_ref,
        } => rollback_aggregate_key_value(household_id, rollback_ref),
        PolicyEventScope::Audit {
            household_id,
            audit_reference_id,
            source_document_id,
            policy_version,
        } => audit_aggregate_key_value(
            household_id,
            audit_reference_id,
            source_document_id,
            policy_version,
        ),
    }
}

fn source_document_aggregate_key_value(
    household_id: &PolicyHouseholdId,
    source_document_id: &ParentPolicyDocumentId,
    policy_version: &PolicyVersion,
) -> String {
    aggregate_key_value(&[
        "policy-source",
        household_id.as_str(),
        source_document_id.as_str(),
        &policy_version.value().to_string(),
    ])
}

fn request_aggregate_key_value(
    household_id: &PolicyHouseholdId,
    request_id: &PolicyRequestId,
    policy_version: &PolicyVersion,
) -> String {
    aggregate_key_value(&[
        "policy-request",
        household_id.as_str(),
        request_id.as_str(),
        &policy_version.value().to_string(),
    ])
}

fn approval_aggregate_key_value(
    household_id: &PolicyHouseholdId,
    approval_id: &PolicyApprovalId,
    request_id: &PolicyRequestId,
    policy_version: &PolicyVersion,
) -> String {
    aggregate_key_value(&[
        "policy-approval",
        household_id.as_str(),
        approval_id.as_str(),
        request_id.as_str(),
        &policy_version.value().to_string(),
    ])
}

fn override_aggregate_key_value(
    household_id: &PolicyHouseholdId,
    override_id: &PolicyOverrideId,
    approval_id: &PolicyApprovalId,
    request_id: &PolicyRequestId,
    policy_version: &PolicyVersion,
) -> String {
    aggregate_key_value(&[
        "policy-override",
        household_id.as_str(),
        override_id.as_str(),
        approval_id.as_str(),
        request_id.as_str(),
        &policy_version.value().to_string(),
    ])
}

fn delivery_aggregate_key_value(
    household_id: &PolicyHouseholdId,
    delivery_id: &PolicyDeliveryId,
    child_profile_id: &PolicyChildProfileId,
    device_id: &PolicyDeviceId,
    domain: PolicyConsumerDomain,
    policy_version: &PolicyVersion,
) -> String {
    aggregate_key_value(&[
        "policy-delivery",
        household_id.as_str(),
        delivery_id.as_str(),
        child_profile_id.as_str(),
        device_id.as_str(),
        super::scope_value::policy_event_domain_name(domain),
        &policy_version.value().to_string(),
    ])
}

fn rollback_aggregate_key_value(
    household_id: &PolicyHouseholdId,
    rollback_ref: &PolicyRollbackRef,
) -> String {
    aggregate_key_value(&[
        "policy-rollback",
        household_id.as_str(),
        rollback_ref.rolled_back_document_id.as_str(),
        &rollback_ref.rolled_back_policy_version.value().to_string(),
        rollback_ref.restored_document_id.as_str(),
        &rollback_ref.restored_policy_version.value().to_string(),
    ])
}

fn audit_aggregate_key_value(
    household_id: &PolicyHouseholdId,
    audit_reference_id: &PolicyAuditReferenceId,
    source_document_id: &ParentPolicyDocumentId,
    policy_version: &PolicyVersion,
) -> String {
    aggregate_key_value(&[
        "policy-audit",
        household_id.as_str(),
        audit_reference_id.as_str(),
        source_document_id.as_str(),
        &policy_version.value().to_string(),
    ])
}

fn aggregate_key_value(parts: &[&str]) -> String {
    let mut value = String::new();
    for (index, part) in parts.iter().enumerate() {
        if index > 0 {
            value.push(':');
        }
        value.push_str(part);
    }
    value
}
