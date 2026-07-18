#![forbid(unsafe_code)]

use super::{
    EventingError, PolicyDeliveryExecutionReceipt, PolicyDeliveryRecord, PolicyDeliveryTransition,
};
use crate::policy_delivery::PolicyDeliveryState;
use crate::policy_delivery::{policy_control, state_values};

const ROLLED_BACK_REFERENCE_STATES: &[PolicyDeliveryState] = &[
    PolicyDeliveryState::Delivered,
    PolicyDeliveryState::Acknowledged,
    PolicyDeliveryState::Applied,
    PolicyDeliveryState::PartialDomainApply,
    PolicyDeliveryState::Degraded,
    PolicyDeliveryState::Offline,
];

pub(super) fn validate_policy_delivery_receipt_rollback_reference(
    current: &PolicyDeliveryRecord,
    transition: &PolicyDeliveryTransition,
    receipt: &PolicyDeliveryExecutionReceipt,
) -> Result<(), EventingError> {
    if transition.state != PolicyDeliveryState::RolledBack {
        let Some(receipt_reference_state) = receipt.rollback_reference_state else {
            return Ok(());
        };

        return Err(EventingError::InvalidValue {
            field: policy_control::delivery::FIELD_ROLLBACK_REFERENCE_STATE,
            value: format!(
                "unexpected rollback reference state {} in execution receipt",
                state_values::policy_delivery_state_name(receipt_reference_state)
            ),
        });
    }

    let Some(transition_reference_state) = transition.rollback_reference_state else {
        return Err(EventingError::InvalidValue {
            field: policy_control::delivery::FIELD_ROLLBACK_REFERENCE_STATE,
            value: String::from("missing rollback reference state for rolled-back receipt"),
        });
    };

    let Some(receipt_reference_state) = receipt.rollback_reference_state else {
        return Err(EventingError::InvalidValue {
            field: policy_control::delivery::FIELD_ROLLBACK_REFERENCE_STATE,
            value: String::from("missing rollback reference state for rolled-back receipt"),
        });
    };

    if transition_reference_state != current.state {
        return Err(EventingError::InvalidValue {
            field: policy_control::delivery::FIELD_ROLLBACK_REFERENCE_STATE,
            value: format!(
                "expected rollback reference state {} but transition reported {}",
                state_values::policy_delivery_state_name(current.state),
                state_values::policy_delivery_state_name(transition_reference_state)
            ),
        });
    }

    if receipt_reference_state != current.state {
        return Err(EventingError::InvalidValue {
            field: policy_control::delivery::FIELD_ROLLBACK_REFERENCE_STATE,
            value: format!(
                "expected rollback reference state {} but receipt reported {}",
                state_values::policy_delivery_state_name(current.state),
                state_values::policy_delivery_state_name(receipt_reference_state)
            ),
        });
    }

    if !ROLLED_BACK_REFERENCE_STATES.contains(&current.state) {
        return Err(EventingError::InvalidValue {
            field: policy_control::delivery::FIELD_ROLLBACK_REFERENCE_STATE,
            value: format!(
                "invalid rollback source state {} for rolled-back transition",
                state_values::policy_delivery_state_name(current.state)
            ),
        });
    }

    Ok(())
}
