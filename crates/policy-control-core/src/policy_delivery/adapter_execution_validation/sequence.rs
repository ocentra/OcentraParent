#![forbid(unsafe_code)]

use std::cmp::Ordering;

use super::{
    EventingError, PolicyDeliveryExecutionReceipt, PolicyDeliveryRecord, PolicyDeliveryTransition,
};
use crate::policy_delivery::policy_control;

pub(super) fn validate_policy_delivery_receipt_sequence(
    current: &PolicyDeliveryRecord,
    transition: &PolicyDeliveryTransition,
    receipt: &PolicyDeliveryExecutionReceipt,
) -> Result<(), EventingError> {
    validate_receipt_sequence_alignment(transition, receipt)?;

    match receipt.sequence.value().cmp(&current.last_sequence.value()) {
        Ordering::Less => stale_execution_receipt(receipt, current),
        Ordering::Equal => validate_current_sequence_replay(current, transition, receipt),
        Ordering::Greater => Ok(()),
    }
}

fn validate_receipt_sequence_alignment(
    transition: &PolicyDeliveryTransition,
    receipt: &PolicyDeliveryExecutionReceipt,
) -> Result<(), EventingError> {
    if receipt.sequence != transition.sequence {
        return Err(EventingError::InvalidValue {
            field: policy_control::delivery::FIELD_SEQUENCE,
            value: format!(
                "expected receipt sequence {} but receipt reported {}",
                transition.sequence.value(),
                receipt.sequence.value()
            ),
        });
    }

    Ok(())
}

fn is_duplicate_execution_receipt(
    current: &PolicyDeliveryRecord,
    transition: &PolicyDeliveryTransition,
    receipt: &PolicyDeliveryExecutionReceipt,
) -> bool {
    transition.sequence == current.last_sequence
        && transition.attempt_id == current.last_attempt_id
        && transition.state == current.state
        && transition.audit_reference_ids == current.audit_reference_ids
        && transition.reason_code == current.reason_code
        && transition.superseded_by_policy_version == current.superseded_by_policy_version
        && transition.rollback_reference_state == current.rollback_reference_state
        && current.execution_receipt() == Some(receipt)
}

fn stale_execution_receipt(
    receipt: &PolicyDeliveryExecutionReceipt,
    current: &PolicyDeliveryRecord,
) -> Result<(), EventingError> {
    Err(EventingError::InvalidValue {
        field: policy_control::delivery::FIELD_SEQUENCE,
        value: format!(
            "execution receipt sequence mismatch: expected=greater-than-current({}), reported={} (stale)",
            current.last_sequence.value(),
            receipt.sequence.value(),
        ),
    })
}

fn validate_current_sequence_replay(
    current: &PolicyDeliveryRecord,
    transition: &PolicyDeliveryTransition,
    receipt: &PolicyDeliveryExecutionReceipt,
) -> Result<(), EventingError> {
    if is_duplicate_execution_receipt(current, transition, receipt) {
        return Ok(());
    }

    Err(EventingError::InvalidValue {
        field: policy_control::delivery::FIELD_SEQUENCE,
        value: format!(
            "execution receipt replay conflict: expected=current-record-provenance, reported=mismatched-receipt-provenance at sequence {}",
            receipt.sequence.value()
        ),
    })
}
