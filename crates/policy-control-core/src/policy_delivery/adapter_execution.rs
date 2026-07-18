#![forbid(unsafe_code)]

use super::{
    adapter_execution_validation, transitions, EventingError, PolicyDeliveryAdapterExecution,
    PolicyDeliveryApplyOutcome, PolicyDeliveryExecutionReceipt, PolicyDeliveryRecord,
    PolicyDeliveryTransition,
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
    transitions::apply_policy_delivery_transition_after_execution_validation(
        current,
        execution.transition,
    )
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
