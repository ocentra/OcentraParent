#![forbid(unsafe_code)]

use super::{
    adapter_execution_validation, policy_control, transitions, validation, EventingError,
    PolicyDeliveryAdapterExecution, PolicyDeliveryApplyOutcome, PolicyDeliveryExecutionReceipt,
    PolicyDeliveryRecord, PolicyDeliveryTransition,
};

pub(super) fn validate_policy_delivery_adapter_execution(
    current: &PolicyDeliveryRecord,
    execution: &PolicyDeliveryAdapterExecution,
) -> Result<(), EventingError> {
    adapter_execution_validation::validate_policy_delivery_execution_receipt(
        current,
        &execution.transition,
        Some(&execution.receipt),
    )?;

    transitions::apply_policy_delivery_transition_after_execution_validation(
        current,
        execution.transition.clone(),
        execution.receipt.clone(),
    )
    .map(|_| ())
}

pub(super) fn validate_policy_delivery_execution_receipt(
    current: &PolicyDeliveryRecord,
    transition: &PolicyDeliveryTransition,
    receipt: Option<&PolicyDeliveryExecutionReceipt>,
) -> Result<(), EventingError> {
    adapter_execution_validation::validate_policy_delivery_execution_receipt(
        current, transition, receipt,
    )
}

pub(super) fn apply_policy_delivery_adapter_execution(
    current: &PolicyDeliveryRecord,
    execution: PolicyDeliveryAdapterExecution,
) -> Result<PolicyDeliveryApplyOutcome, EventingError> {
    validate_policy_delivery_adapter_execution(current, &execution)?;
    let PolicyDeliveryAdapterExecution {
        transition,
        receipt,
    } = execution;
    transitions::apply_policy_delivery_transition_after_execution_validation(
        current, transition, receipt,
    )
}

pub(super) fn replay_policy_delivery_execution_receipt(
    current: &PolicyDeliveryRecord,
    transition: &PolicyDeliveryTransition,
    receipt: &PolicyDeliveryExecutionReceipt,
) -> Result<PolicyDeliveryApplyOutcome, EventingError> {
    validation::validate_policy_delivery_record(current)?;
    validation::validate_policy_delivery_transition(transition, current.policy_version)?;
    if transition.sequence != current.last_sequence {
        return Err(EventingError::InvalidValue {
            field: policy_control::delivery::FIELD_SEQUENCE,
            value: format!(
                "execution receipt retry requires current sequence {}, reported {}",
                current.last_sequence.value(),
                transition.sequence.value()
            ),
        });
    }
    adapter_execution_validation::validate_policy_delivery_execution_receipt(
        current,
        transition,
        Some(receipt),
    )?;
    Ok(PolicyDeliveryApplyOutcome::Duplicate(current.clone()))
}
