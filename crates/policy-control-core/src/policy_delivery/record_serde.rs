#![forbid(unsafe_code)]

use serde::{Deserialize, Deserializer};

use super::{
    policy_control, validation, EventingError, ParentPolicyDocumentId, PolicyAuditReferenceId,
    PolicyDeliveryAttemptId, PolicyDeliveryExecutionReceipt, PolicyDeliveryId,
    PolicyDeliveryReceiptProvenance, PolicyDeliveryRecord, PolicyDeliverySequence,
    PolicyDeliveryState, PolicyDeliveryTarget, PolicyHouseholdId, PolicyReasonCode,
    PolicyRollbackRef, PolicyVersion, SchemaVersion,
};

#[derive(Deserialize)]
struct PolicyDeliveryRecordWire {
    schema_version: SchemaVersion,
    delivery_id: PolicyDeliveryId,
    household_id: PolicyHouseholdId,
    policy_version: PolicyVersion,
    source_document_id: ParentPolicyDocumentId,
    target: PolicyDeliveryTarget,
    state: PolicyDeliveryState,
    last_sequence: PolicyDeliverySequence,
    last_attempt_id: PolicyDeliveryAttemptId,
    audit_reference_ids: Vec<PolicyAuditReferenceId>,
    #[serde(default)]
    source_audit_reference_ids: Vec<PolicyAuditReferenceId>,
    #[serde(default)]
    source_superseded_by_policy_version: Option<PolicyVersion>,
    #[serde(default)]
    source_rollback_ref: Option<PolicyRollbackRef>,
    reason_code: Option<PolicyReasonCode>,
    superseded_by_policy_version: Option<PolicyVersion>,
    rollback_reference_state: Option<PolicyDeliveryState>,
    #[serde(default)]
    execution_receipt: Option<PolicyDeliveryExecutionReceipt>,
}

impl From<PolicyDeliveryRecordWire> for PolicyDeliveryRecord {
    fn from(wire: PolicyDeliveryRecordWire) -> Self {
        Self {
            schema_version: wire.schema_version,
            delivery_id: wire.delivery_id,
            household_id: wire.household_id,
            policy_version: wire.policy_version,
            source_document_id: wire.source_document_id,
            target: wire.target,
            state: wire.state,
            last_sequence: wire.last_sequence,
            last_attempt_id: wire.last_attempt_id,
            audit_reference_ids: wire.audit_reference_ids,
            source_audit_reference_ids: wire.source_audit_reference_ids,
            source_superseded_by_policy_version: wire.source_superseded_by_policy_version,
            source_rollback_ref: wire.source_rollback_ref,
            reason_code: wire.reason_code,
            superseded_by_policy_version: wire.superseded_by_policy_version,
            rollback_reference_state: wire.rollback_reference_state,
            execution_receipt: wire.execution_receipt,
        }
    }
}

impl<'de> Deserialize<'de> for PolicyDeliveryRecord {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let record = Self::from(PolicyDeliveryRecordWire::deserialize(deserializer)?);
        validate_untrusted_hydration(&record).map_err(serde::de::Error::custom)?;
        Ok(record)
    }
}

fn validate_untrusted_hydration(record: &PolicyDeliveryRecord) -> Result<(), EventingError> {
    let current_schema_version = validation::policy_delivery_schema_version()?;
    if record.schema_version > current_schema_version {
        return Err(EventingError::InvalidValue {
            field: "policy_delivery.schema_version",
            value: format!(
                "unsupported future schema version {}",
                record.schema_version.value()
            ),
        });
    }
    let legacy_unverified = record.execution_receipt_provenance()
        == PolicyDeliveryReceiptProvenance::LegacySchemaV1Unverified;
    if matches!(
        record.state,
        PolicyDeliveryState::Acknowledged
            | PolicyDeliveryState::Applied
            | PolicyDeliveryState::RolledBack
    ) && !legacy_unverified
    {
        return Err(EventingError::InvalidValue {
            field: policy_control::delivery::FIELD_STATE,
            value: "generic receipt-required record hydration is unsupported".to_string(),
        });
    }
    if legacy_unverified {
        validate_legacy_schema_v1_unverified_record(record)
    } else {
        validation::validate_policy_delivery_record(record)
    }
}

fn validate_legacy_schema_v1_unverified_record(
    record: &PolicyDeliveryRecord,
) -> Result<(), EventingError> {
    validation::validate_policy_delivery_record(record)
}
