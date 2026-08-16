#![forbid(unsafe_code)]

use super::{
    identity, rollback, sequence, EventingError, PolicyDeliveryExecutionReceipt,
    PolicyDeliveryRecord, PolicyDeliveryTransition,
};
use crate::policy_delivery::PolicyDeliveryState;
use crate::policy_delivery::{policy_control, state_values};

#[derive(Clone, Copy, PartialEq, Eq)]
enum ReceiptRequirement {
    Required,
    Forbidden,
}

pub(super) fn validate_policy_delivery_execution_receipt(
    current: &PolicyDeliveryRecord,
    transition: &PolicyDeliveryTransition,
    receipt: Option<&PolicyDeliveryExecutionReceipt>,
) -> Result<(), EventingError> {
    match receipt_requirement(transition.state) {
        ReceiptRequirement::Required => {
            let receipt =
                receipt.ok_or_else(|| missing_adapter_execution_receipt(transition.state))?;
            identity::validate_policy_delivery_receipt_identity(current, transition, receipt)?;
            sequence::validate_policy_delivery_receipt_sequence(current, transition, receipt)?;
            rollback::validate_policy_delivery_receipt_rollback_reference(
                current, transition, receipt,
            )
        }
        ReceiptRequirement::Forbidden => {
            unexpected_adapter_execution_receipt(transition.state, receipt)
        }
    }
}

fn receipt_requirement(state: PolicyDeliveryState) -> ReceiptRequirement {
    if matches!(
        state,
        PolicyDeliveryState::Acknowledged
            | PolicyDeliveryState::Applied
            | PolicyDeliveryState::RolledBack
    ) {
        ReceiptRequirement::Required
    } else {
        ReceiptRequirement::Forbidden
    }
}

fn missing_adapter_execution_receipt(state: PolicyDeliveryState) -> EventingError {
    EventingError::InvalidValue {
        field: policy_control::delivery::FIELD_STATE,
        value: format!(
            "missing adapter execution receipt for {}",
            state_values::policy_delivery_state_name(state)
        ),
    }
}

fn unexpected_adapter_execution_receipt(
    state: PolicyDeliveryState,
    receipt: Option<&PolicyDeliveryExecutionReceipt>,
) -> Result<(), EventingError> {
    if let Some(receipt) = receipt {
        return Err(EventingError::InvalidValue {
            field: policy_control::delivery::FIELD_STATE,
            value: format!(
                "unexpected adapter execution receipt for {} with receipt sequence {}",
                state_values::policy_delivery_state_name(state),
                receipt.sequence.value()
            ),
        });
    }

    Ok(())
}
