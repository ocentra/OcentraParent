#![forbid(unsafe_code)]

use std::cmp::Ordering;

use super::{
    adapter_execution_validation, policy_control, transitions, EventingError,
    PolicyDeliveryAdapterExecution, PolicyDeliveryApplyOutcome, PolicyDeliveryExecutionReceipt,
    PolicyDeliveryRecord, PolicyDeliveryTransition,
};

pub(super) fn validate_policy_delivery_adapter_execution(
    current: &PolicyDeliveryRecord,
    execution: &PolicyDeliveryAdapterExecution,
) -> Result<(), EventingError> {
    validate_policy_delivery_execution_receipt(
        current,
        &execution.transition,
        Some(&execution.receipt),
    )
}

pub(super) fn validate_policy_delivery_execution_receipt(
    current: &PolicyDeliveryRecord,
    transition: &PolicyDeliveryTransition,
    receipt: Option<&PolicyDeliveryExecutionReceipt>,
) -> Result<(), EventingError> {
    let Some(receipt) = receipt else {
        return Err(EventingError::InvalidValue {
            field: policy_control::delivery::FIELD_STATE,
            value: format!(
                "missing adapter execution receipt for {}",
                super::state_values::policy_delivery_state_name(transition.state)
            ),
        });
    };

    adapter_execution_validation::validate_policy_delivery_receipt_identity(
        current, transition, receipt,
    )?;
    validate_policy_delivery_receipt_sequence(current, transition, receipt)?;
    adapter_execution_validation::validate_policy_delivery_receipt_rollback_reference(
        transition, receipt,
    )
}

pub(super) fn apply_policy_delivery_adapter_execution(
    current: &PolicyDeliveryRecord,
    execution: PolicyDeliveryAdapterExecution,
) -> Result<PolicyDeliveryApplyOutcome, EventingError> {
    validate_policy_delivery_adapter_execution(current, &execution)?;
    transitions::apply_policy_delivery_transition(current, execution.transition)
}

pub(super) fn apply_policy_delivery_execution_transition(
    current: &PolicyDeliveryRecord,
    transition: PolicyDeliveryTransition,
    receipt: PolicyDeliveryExecutionReceipt,
) -> Result<PolicyDeliveryApplyOutcome, EventingError> {
    apply_policy_delivery_adapter_execution(
        current,
        PolicyDeliveryAdapterExecution {
            transition,
            receipt,
        },
    )
}

fn validate_policy_delivery_receipt_sequence(
    current: &PolicyDeliveryRecord,
    transition: &PolicyDeliveryTransition,
    receipt: &PolicyDeliveryExecutionReceipt,
) -> Result<(), EventingError> {
    match receipt.sequence.value().cmp(&current.last_sequence.value()) {
        Ordering::Less => stale_execution_receipt(receipt, current),
        Ordering::Equal => validate_duplicate_execution_receipt(current, receipt),
        Ordering::Greater => validate_receipt_sequence_alignment(transition, receipt),
    }
}

fn is_duplicate_execution_receipt(
    current: &PolicyDeliveryRecord,
    receipt: &PolicyDeliveryExecutionReceipt,
) -> bool {
    receipt.sequence == current.last_sequence
        && receipt.delivery_id == current.delivery_id
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

fn validate_duplicate_execution_receipt(
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

    Ok(())
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
