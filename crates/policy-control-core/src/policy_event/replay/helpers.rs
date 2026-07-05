#![forbid(unsafe_code)]

use ocentra_eventing::contract_registry::EventContractRegistry;
use ocentra_eventing::envelope::DomainEvent;
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{
    AggregateKey, EventNamespace, EventType, IdempotencyKey, SchemaVersion,
};
use ocentra_eventing::topology::EventTopologyFamilyVariant;
use ocentra_parent_agent_protocol::constants::policy_control;

use crate::policy_delivery::PolicyDeliveryId;
use crate::policy_request::{PolicyApprovalId, PolicyOverrideId, PolicyRequestId};
use crate::policy_source::{
    ParentPolicyDocumentId, PolicyAuditReferenceId, PolicyChildProfileId, PolicyConsumerDomain,
    PolicyDeviceId, PolicyHouseholdId, PolicyReasonCode, PolicyRollbackRef, PolicyVersion,
};

use super::{
    PolicyEvent, PolicyEventApplyOutcome, PolicyEventDeadLetterReason, PolicyEventKind,
    PolicyEventReplayRecord, PolicyEventScope,
};

mod sample;
mod scope;

const POLICY_EVENT_SCHEMA_VERSION_VALUE: u16 = 1;
const POLICY_EVENT_NAMESPACE_VALUE: &str = "policy";

pub(crate) fn policy_event_redacted_summary(event: &PolicyEvent) -> String {
    let mut value = String::from("policy-event kind=");
    value.push_str(event.kind.event_type_name());
    value.push_str(" scope=");
    value.push_str(policy_event_scope_family_label(&event.scope));
    value.push_str(" sequence=");
    value.push_str(&event.sequence.value().to_string());
    if matches!(event.kind, PolicyEventKind::ManualRequired) {
        value.push_str(" manual-required");
    }
    if matches!(event.kind, PolicyEventKind::DeadLetterRecorded) {
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
    policy_event_scope_aggregate_key(&event.scope)
}

pub(crate) fn policy_event_idempotency_key(
    event: &PolicyEvent,
) -> Result<IdempotencyKey, EventingError> {
    IdempotencyKey::parse(policy_event_idempotency_key_value(event)?)
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

pub(crate) fn policy_event_schema_version() -> Result<SchemaVersion, EventingError> {
    SchemaVersion::new(POLICY_EVENT_SCHEMA_VERSION_VALUE)
}

pub(crate) fn policy_event_family_namespace() -> Result<EventNamespace, EventingError> {
    EventNamespace::parse(POLICY_EVENT_NAMESPACE_VALUE)
}

pub(crate) fn policy_event_family_variants(
) -> Result<Vec<EventTopologyFamilyVariant>, EventingError> {
    let family = policy_event_family_namespace()?;
    crate::policy_event::POLICY_EVENT_KINDS
        .iter()
        .copied()
        .map(|kind| {
            Ok(EventTopologyFamilyVariant {
                family: family.clone(),
                event_type: EventType::parse(kind.event_type_name())?,
            })
        })
        .collect()
}

pub(crate) fn policy_event_contract_registry() -> Result<EventContractRegistry, EventingError> {
    let mut registry = EventContractRegistry::new();
    for kind in crate::policy_event::POLICY_EVENT_KINDS.iter().copied() {
        let event = sample_policy_event(kind)?;
        registry.register_event(&event)?;
    }
    Ok(registry)
}

pub(crate) fn apply_policy_event_replay(
    current: &PolicyEventReplayRecord,
    next: &PolicyEvent,
) -> Result<PolicyEventApplyOutcome, EventingError> {
    let next_aggregate_key = policy_event_aggregate_key(next)?;
    let next_idempotency_key = policy_event_idempotency_key(next)?;

    assert_matching_aggregate_key(current, &next_aggregate_key)?;

    match next.sequence.value().cmp(&current.last_sequence.value()) {
        std::cmp::Ordering::Less => Ok(PolicyEventApplyOutcome::Stale(current.clone())),
        std::cmp::Ordering::Equal => {
            apply_equal_sequence_replay(current, next, next_idempotency_key)
        }
        std::cmp::Ordering::Greater => Ok(PolicyEventApplyOutcome::Advanced(
            advanced_replay_record(next, next_aggregate_key, next_idempotency_key)?,
        )),
    }
}

pub(crate) fn sample_policy_event(kind: PolicyEventKind) -> Result<PolicyEvent, EventingError> {
    sample::sample_policy_event(kind)
}

pub(crate) fn sample_policy_event_scope(
    kind: PolicyEventKind,
) -> Result<PolicyEventScope, EventingError> {
    scope::sample_policy_event_scope(kind)
}

pub(crate) fn kind_requires_reason(kind: PolicyEventKind) -> bool {
    sample::kind_requires_reason(kind)
}

pub(crate) fn policy_event_kind_reason_code_value(kind: PolicyEventKind) -> &'static str {
    sample::policy_event_kind_reason_code_value(kind)
}

pub(crate) fn policy_event_idempotency_key_value(
    event: &PolicyEvent,
) -> Result<String, EventingError> {
    let aggregate_key = policy_event_aggregate_key(event)?;
    let mut value = String::from("policy-event:");
    value.push_str(event.kind.event_type_name());
    value.push('|');
    value.push_str(aggregate_key.as_str());
    value.push('|');
    value.push_str(&event.sequence.value().to_string());
    value.push('|');
    value.push_str(policy_event_scope_family_label(&event.scope));
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

pub(crate) fn policy_event_scope_family_label(scope: &PolicyEventScope) -> &'static str {
    match scope {
        PolicyEventScope::SourceDocument { .. } => "source-document",
        PolicyEventScope::Request { .. } => "request",
        PolicyEventScope::Approval { .. } => "approval",
        PolicyEventScope::Override { .. } => "override",
        PolicyEventScope::Delivery { .. } => "delivery",
        PolicyEventScope::Rollback { .. } => "rollback",
        PolicyEventScope::Audit { .. } => "audit",
    }
}

pub(crate) fn policy_event_scope_aggregate_key(
    scope: &PolicyEventScope,
) -> Result<AggregateKey, EventingError> {
    AggregateKey::parse(policy_event_scope_aggregate_key_value(scope))
}

pub(crate) fn policy_event_scope_aggregate_key_value(scope: &PolicyEventScope) -> String {
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
        policy_event_domain_name(domain),
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

fn policy_event_domain_name(domain: PolicyConsumerDomain) -> &'static str {
    match domain {
        PolicyConsumerDomain::App => "app",
        PolicyConsumerDomain::Browser => "browser",
        PolicyConsumerDomain::Network => "network",
        PolicyConsumerDomain::Tracking => "tracking",
        PolicyConsumerDomain::Screen => "screen",
        PolicyConsumerDomain::Ai => "ai",
    }
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

fn conflicting_replay_value(
    sequence: crate::policy_event::PolicyEventSequence,
    last_event_type: &EventType,
) -> String {
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
        last_event_type: policy_event_event_type(next)?,
        last_idempotency_key: idempotency_key,
    })
}

fn apply_equal_sequence_replay(
    current: &PolicyEventReplayRecord,
    next: &PolicyEvent,
    next_idempotency_key: IdempotencyKey,
) -> Result<PolicyEventApplyOutcome, EventingError> {
    if next_idempotency_key == current.last_idempotency_key
        && policy_event_event_type(next)? == current.last_event_type
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
