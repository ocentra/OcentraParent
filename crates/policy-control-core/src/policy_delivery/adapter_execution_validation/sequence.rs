#![forbid(unsafe_code)]

use std::cmp::Ordering;

use super::{
    EventingError, PolicyDeliveryExecutionReceipt, PolicyDeliveryRecord, PolicyDeliveryTransition,
};
use crate::policy_delivery::{policy_control, state_values};

pub(super) fn validate_policy_delivery_receipt_sequence(
    current: &PolicyDeliveryRecord,
    transition: &PolicyDeliveryTransition,
    receipt: &PolicyDeliveryExecutionReceipt,
) -> Result<(), EventingError> {
    validate_receipt_sequence_alignment(transition, receipt)?;

    match receipt.sequence.value().cmp(&current.last_sequence.value()) {
        Ordering::Less => stale_execution_receipt(receipt, current),
        Ordering::Equal => validate_current_sequence_replay(current, receipt),
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
    receipt: &PolicyDeliveryExecutionReceipt,
) -> bool {
    receipt.sequence == current.last_sequence
        && receipt.delivery_id == current.delivery_id
        && receipt.household_id == current.household_id
        && receipt.policy_version == current.policy_version
        && receipt.target == current.target
        && receipt.attempt_id == current.last_attempt_id
        && receipt.state == current.state
        && receipt.audit_reference_ids == current.audit_reference_ids
        && receipt.rollback_reference_state == current.rollback_reference_state
}

fn stale_execution_receipt(
    receipt: &PolicyDeliveryExecutionReceipt,
    current: &PolicyDeliveryRecord,
) -> Result<(), EventingError> {
    Err(EventingError::InvalidValue {
        field: policy_control::delivery::FIELD_SEQUENCE,
        value: format!(
            "stale execution receipt for sequence {} on {}",
            receipt.sequence.value(),
            current.delivery_id.as_str()
        ),
    })
}

fn validate_current_sequence_replay(
    current: &PolicyDeliveryRecord,
    receipt: &PolicyDeliveryExecutionReceipt,
) -> Result<(), EventingError> {
    if is_duplicate_execution_receipt(current, receipt) {
        return Err(EventingError::InvalidValue {
            field: policy_control::delivery::FIELD_SEQUENCE,
            value: format!(
                "duplicate execution receipt for sequence {} on {}",
                receipt.sequence.value(),
                current.delivery_id.as_str()
            ),
        });
    }

    Err(EventingError::InvalidValue {
        field: policy_control::delivery::FIELD_SEQUENCE,
        value: state_values::conflicting_replay_value(receipt.sequence, &current.delivery_id),
    })
}
