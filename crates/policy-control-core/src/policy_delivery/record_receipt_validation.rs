#![forbid(unsafe_code)]

use super::{
    policy_control, state_values, EventingError, PolicyDeliveryExecutionReceipt,
    PolicyDeliveryParentVisibleState, PolicyDeliveryReceiptProvenance, PolicyDeliveryRecord,
    PolicyDeliveryState,
};

const FIELD_TARGET_DOMAIN: &str = "policy_delivery.target.domain";
const RECORD_RECEIPT_MISMATCH: &str =
    "delivery record receipt evidence mismatch: expected=record, reported=execution-receipt";

pub(super) fn parent_visible_state(
    record: &PolicyDeliveryRecord,
) -> PolicyDeliveryParentVisibleState {
    if record.execution_receipt_provenance()
        == PolicyDeliveryReceiptProvenance::LegacySchemaV1Unverified
        || (record.state == PolicyDeliveryState::Applied && !record.is_active())
    {
        PolicyDeliveryParentVisibleState::ManualRequired
    } else {
        state_values::policy_delivery_parent_visible_state(record.state)
    }
}

pub(super) fn validate(record: &PolicyDeliveryRecord) -> Result<(), EventingError> {
    let Some(receipt) = required_receipt(record)? else {
        return Ok(());
    };
    let mismatch = delivery_identity_mismatch(record, receipt)
        .or_else(|| execution_identity_mismatch(record, receipt));
    if let Some(field) = mismatch {
        return Err(EventingError::InvalidValue {
            field,
            value: RECORD_RECEIPT_MISMATCH.to_string(),
        });
    }
    Ok(())
}

fn required_receipt(
    record: &PolicyDeliveryRecord,
) -> Result<Option<&PolicyDeliveryExecutionReceipt>, EventingError> {
    let required = matches!(
        record.state,
        PolicyDeliveryState::Acknowledged
            | PolicyDeliveryState::Applied
            | PolicyDeliveryState::RolledBack
    );
    match (required, record.execution_receipt.as_ref()) {
        (true, Some(receipt)) => Ok(Some(receipt)),
        (true, None)
            if record.execution_receipt_provenance()
                == PolicyDeliveryReceiptProvenance::LegacySchemaV1Unverified =>
        {
            Ok(None)
        }
        (true, None) => Err(EventingError::InvalidValue {
            field: policy_control::delivery::FIELD_STATE,
            value: format!(
                "missing adapter execution receipt for {}",
                state_values::policy_delivery_state_name(record.state)
            ),
        }),
        (false, Some(_)) => Err(EventingError::InvalidValue {
            field: policy_control::delivery::FIELD_STATE,
            value: format!(
                "unexpected stored execution receipt for {}",
                state_values::policy_delivery_state_name(record.state)
            ),
        }),
        (false, None) => Ok(None),
    }
}

fn delivery_identity_mismatch(
    record: &PolicyDeliveryRecord,
    receipt: &PolicyDeliveryExecutionReceipt,
) -> Option<&'static str> {
    [
        (
            receipt.delivery_id != record.delivery_id,
            policy_control::delivery::FIELD_DELIVERY_ID,
        ),
        (
            receipt.household_id != record.household_id,
            policy_control::source::FIELD_HOUSEHOLD_ID,
        ),
        (
            receipt.policy_version != record.policy_version,
            policy_control::source::FIELD_POLICY_VERSION,
        ),
        (
            receipt.source_document_id != record.source_document_id,
            policy_control::source::FIELD_DOCUMENT_ID,
        ),
        (
            receipt.target.child_profile_id != record.target.child_profile_id,
            policy_control::source::FIELD_CHILD_PROFILE_ID,
        ),
        (
            receipt.target.device_id != record.target.device_id,
            policy_control::source::FIELD_DEVICE_ID,
        ),
        (
            receipt.target.domain != record.target.domain,
            FIELD_TARGET_DOMAIN,
        ),
    ]
    .into_iter()
    .find_map(|(mismatch, field)| mismatch.then_some(field))
}

fn execution_identity_mismatch(
    record: &PolicyDeliveryRecord,
    receipt: &PolicyDeliveryExecutionReceipt,
) -> Option<&'static str> {
    [
        (
            receipt.attempt_id != record.last_attempt_id,
            policy_control::delivery::FIELD_ATTEMPT_ID,
        ),
        (
            receipt.sequence != record.last_sequence,
            policy_control::delivery::FIELD_SEQUENCE,
        ),
        (
            receipt.state != record.state,
            policy_control::delivery::FIELD_STATE,
        ),
        (
            receipt.audit_reference_ids != record.audit_reference_ids,
            policy_control::delivery::FIELD_AUDIT_REFERENCE_IDS,
        ),
        (
            receipt.reason_code != record.reason_code,
            policy_control::delivery::FIELD_REASON_CODE,
        ),
        (
            receipt.rollback_reference_state != record.rollback_reference_state,
            policy_control::delivery::FIELD_ROLLBACK_REFERENCE_STATE,
        ),
    ]
    .into_iter()
    .find_map(|(mismatch, field)| mismatch.then_some(field))
}
