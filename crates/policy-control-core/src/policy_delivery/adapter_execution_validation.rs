#![forbid(unsafe_code)]

use super::{
    policy_control, state_values, EventingError, PolicyDeliveryExecutionReceipt,
    PolicyDeliveryRecord, PolicyDeliveryState, PolicyDeliveryTransition,
};

pub(super) fn validate_policy_delivery_receipt_identity(
    current: &PolicyDeliveryRecord,
    transition: &PolicyDeliveryTransition,
    receipt: &PolicyDeliveryExecutionReceipt,
) -> Result<(), EventingError> {
    let mismatch = [
        (
            receipt.delivery_id != current.delivery_id,
            policy_control::delivery::FIELD_DELIVERY_ID,
            format!(
                "expected delivery {} but receipt reported {}",
                current.delivery_id.as_str(),
                receipt.delivery_id.as_str()
            ),
        ),
        (
            receipt.attempt_id != transition.attempt_id,
            policy_control::delivery::FIELD_ATTEMPT_ID,
            format!(
                "expected attempt {} but receipt reported {}",
                transition.attempt_id.as_str(),
                receipt.attempt_id.as_str()
            ),
        ),
        (
            receipt.state != transition.state,
            policy_control::delivery::FIELD_STATE,
            format!(
                "expected receipt state {} but receipt reported {}",
                state_values::policy_delivery_state_name(transition.state),
                state_values::policy_delivery_state_name(receipt.state)
            ),
        ),
        (
            receipt.audit_reference_ids != transition.audit_reference_ids,
            policy_control::delivery::FIELD_AUDIT_REFERENCE_IDS,
            String::from("expected audit references to match execution receipt"),
        ),
    ]
    .into_iter()
    .find(|(mismatch, _, _)| *mismatch);

    if let Some((_, field, value)) = mismatch {
        return Err(EventingError::InvalidValue { field, value });
    }

    Ok(())
}

pub(super) fn validate_policy_delivery_receipt_rollback_reference(
    transition: &PolicyDeliveryTransition,
    receipt: &PolicyDeliveryExecutionReceipt,
) -> Result<(), EventingError> {
    if transition.state != PolicyDeliveryState::RolledBack {
        return if receipt.rollback_reference_state.is_some() {
            Err(EventingError::InvalidValue {
                field: policy_control::delivery::FIELD_ROLLBACK_REFERENCE_STATE,
                value: format!(
                    "unexpected rollback reference state {} in execution receipt",
                    state_values::policy_delivery_state_name(
                        receipt.rollback_reference_state.expect("checked is_some"),
                    )
                ),
            })
        } else {
            Ok(())
        };
    }

    let Some(receipt_reference) = receipt.rollback_reference_state else {
        return Err(EventingError::InvalidValue {
            field: policy_control::delivery::FIELD_ROLLBACK_REFERENCE_STATE,
            value: String::from("missing rollback reference state for rolled-back receipt"),
        });
    };

    match transition.rollback_reference_state {
        Some(expected_reference) if receipt_reference == expected_reference => Ok(()),
        Some(expected_reference) => Err(EventingError::InvalidValue {
            field: policy_control::delivery::FIELD_ROLLBACK_REFERENCE_STATE,
            value: format!(
                "expected rollback reference state {} but receipt reported {}",
                state_values::policy_delivery_state_name(expected_reference),
                state_values::policy_delivery_state_name(receipt_reference)
            ),
        }),
        None => Err(EventingError::InvalidValue {
            field: policy_control::delivery::FIELD_ROLLBACK_REFERENCE_STATE,
            value: format!(
                "unexpected rollback reference state {} in execution receipt",
                state_values::policy_delivery_state_name(receipt_reference)
            ),
        }),
    }
}
