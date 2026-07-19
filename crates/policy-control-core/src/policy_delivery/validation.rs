#![forbid(unsafe_code)]

use std::collections::BTreeSet;

use sha2::{Digest, Sha256};

use super::{
    policy_control, state_context, state_values, EventingError, PolicyAuditReferenceId,
    PolicyDeliveryAttemptId, PolicyDeliveryId, PolicyDeliveryRecord, PolicyDeliverySequence,
    PolicyDeliveryTarget, PolicyDeliveryTransition, PolicyVersion, SchemaVersion,
    POLICY_DELIVERY_SCHEMA_VERSION_VALUE,
};
use crate::policy_source::{
    CompiledDomainPolicyArtifact, ParentPolicyDocumentId, PolicyScheduleId,
};

const POLICY_DELIVERY_ID_DOMAIN_SEPARATOR: &str = "policy-delivery-id:v1";
const POLICY_DELIVERY_ID_PREFIX: &str = "policy-delivery:v1:sha256:";
pub(super) fn policy_delivery_schema_version() -> Result<SchemaVersion, EventingError> {
    SchemaVersion::new(POLICY_DELIVERY_SCHEMA_VERSION_VALUE)
}

pub(super) fn validate_policy_delivery_record(
    record: &PolicyDeliveryRecord,
) -> Result<(), EventingError> {
    assert_audit_refs(&record.audit_reference_ids)?;
    state_context::assert_state_context(
        record.state,
        record.reason_code.as_ref(),
        record.superseded_by_policy_version,
        record.rollback_reference_state,
        record.policy_version,
    )?;
    super::record_receipt_validation::validate(record)
}

pub(super) fn validate_policy_delivery_transition(
    transition: &PolicyDeliveryTransition,
    current_policy_version: PolicyVersion,
) -> Result<(), EventingError> {
    assert_audit_refs(&transition.audit_reference_ids)?;
    state_context::assert_state_context(
        transition.state,
        transition.reason_code.as_ref(),
        transition.superseded_by_policy_version,
        transition.rollback_reference_state,
        current_policy_version,
    )
}

pub(super) fn derive_policy_delivery_id(
    artifact: &CompiledDomainPolicyArtifact,
    target: &PolicyDeliveryTarget,
    attempt_id: &PolicyDeliveryAttemptId,
    sequence: PolicyDeliverySequence,
) -> Result<PolicyDeliveryId, EventingError> {
    PolicyDeliveryId::parse(canonical_policy_delivery_id_value(
        artifact, target, attempt_id, sequence,
    ))
}

pub(super) fn validate_policy_delivery_id(
    value: impl Into<String>,
) -> Result<String, EventingError> {
    map_delivery_id_error(
        ParentPolicyDocumentId::parse(value).map(Into::into),
        policy_control::delivery::FIELD_DELIVERY_ID,
    )
}

pub(super) fn validate_policy_delivery_attempt_id(
    value: impl Into<String>,
) -> Result<String, EventingError> {
    map_delivery_id_error(
        PolicyScheduleId::parse(value).map(Into::into),
        policy_control::delivery::FIELD_ATTEMPT_ID,
    )
}

fn map_delivery_id_error<T>(
    value: Result<T, EventingError>,
    field: &'static str,
) -> Result<T, EventingError> {
    match value {
        Ok(value) => Ok(value),
        Err(EventingError::EmptyValue { .. }) => Err(EventingError::EmptyValue { field }),
        Err(error) => Err(error),
    }
}

fn canonical_policy_delivery_id_value(
    artifact: &CompiledDomainPolicyArtifact,
    target: &PolicyDeliveryTarget,
    attempt_id: &PolicyDeliveryAttemptId,
    sequence: PolicyDeliverySequence,
) -> String {
    let mut hasher = Sha256::new();
    update_framed_text(&mut hasher, POLICY_DELIVERY_ID_DOMAIN_SEPARATOR);
    update_framed_text(&mut hasher, artifact.household_id.as_str());
    update_framed_u64(&mut hasher, artifact.policy_version.value());
    update_framed_text(&mut hasher, artifact.source_document_id.as_str());
    update_framed_text(&mut hasher, target.child_profile_id.as_str());
    update_framed_text(&mut hasher, target.device_id.as_str());
    update_framed_text(
        &mut hasher,
        state_values::policy_delivery_domain_name(target.domain),
    );
    update_framed_text(&mut hasher, attempt_id.as_str());
    update_framed_u64(&mut hasher, sequence.value());
    format!("{POLICY_DELIVERY_ID_PREFIX}{:x}", hasher.finalize())
}

fn update_framed_text(hasher: &mut Sha256, value: &str) {
    hasher.update((value.len() as u64).to_be_bytes());
    hasher.update(value.as_bytes());
}

fn update_framed_u64(hasher: &mut Sha256, value: u64) {
    update_framed_text(hasher, &value.to_string());
}

fn assert_audit_refs(audit_reference_ids: &[PolicyAuditReferenceId]) -> Result<(), EventingError> {
    if audit_reference_ids.is_empty() {
        return Err(EventingError::InvalidValue {
            field: policy_control::delivery::FIELD_AUDIT_REFERENCE_IDS,
            value: policy_control::delivery::VALUE_MISSING_AUDIT_REFERENCES.to_string(),
        });
    }

    let mut seen = BTreeSet::new();
    for audit_reference_id in audit_reference_ids {
        if !seen.insert(audit_reference_id.clone()) {
            return Err(EventingError::InvalidValue {
                field: policy_control::delivery::FIELD_AUDIT_REFERENCE_IDS,
                value: String::from("duplicate audit reference"),
            });
        }
    }

    Ok(())
}
