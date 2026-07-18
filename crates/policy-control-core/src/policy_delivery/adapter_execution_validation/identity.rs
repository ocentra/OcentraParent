#![forbid(unsafe_code)]

use super::{
    EventingError, PolicyDeliveryExecutionReceipt, PolicyDeliveryRecord, PolicyDeliveryTransition,
};
use crate::policy_delivery::{policy_control, state_values};
use crate::policy_source::PolicyConsumerDomain;

pub(super) fn validate_policy_delivery_receipt_identity(
    current: &PolicyDeliveryRecord,
    transition: &PolicyDeliveryTransition,
    receipt: &PolicyDeliveryExecutionReceipt,
) -> Result<(), EventingError> {
    validate_policy_delivery_receipt_delivery_identity(current, receipt)?;
    validate_policy_delivery_receipt_execution_identity(transition, receipt)
}

fn validate_policy_delivery_receipt_delivery_identity(
    current: &PolicyDeliveryRecord,
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
            receipt.household_id != current.household_id,
            policy_control::source::FIELD_HOUSEHOLD_ID,
            format!(
                "expected household {} but receipt reported {}",
                current.household_id.as_str(),
                receipt.household_id.as_str()
            ),
        ),
        (
            receipt.policy_version != current.policy_version,
            policy_control::source::FIELD_POLICY_VERSION,
            format!(
                "expected policy version {} but receipt reported {}",
                current.policy_version.value(),
                receipt.policy_version.value()
            ),
        ),
        (
            receipt.target.child_profile_id != current.target.child_profile_id,
            policy_control::source::FIELD_CHILD_PROFILE_ID,
            format!(
                "expected child profile {} but receipt reported {}",
                current.target.child_profile_id.as_str(),
                receipt.target.child_profile_id.as_str()
            ),
        ),
        (
            receipt.target.device_id != current.target.device_id,
            policy_control::source::FIELD_DEVICE_ID,
            format!(
                "expected device {} but receipt reported {}",
                current.target.device_id.as_str(),
                receipt.target.device_id.as_str()
            ),
        ),
        (
            receipt.target.domain != current.target.domain,
            policy_control::delivery::FIELD_STATE,
            format!(
                "expected delivery domain {} but receipt reported {}",
                policy_delivery_domain_name(current.target.domain),
                policy_delivery_domain_name(receipt.target.domain)
            ),
        ),
    ]
    .into_iter()
    .find(|(mismatch, _, _)| *mismatch);

    if let Some((_, field, value)) = mismatch {
        return Err(EventingError::InvalidValue { field, value });
    }

    Ok(())
}

fn validate_policy_delivery_receipt_execution_identity(
    transition: &PolicyDeliveryTransition,
    receipt: &PolicyDeliveryExecutionReceipt,
) -> Result<(), EventingError> {
    let mismatch = [
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

fn policy_delivery_domain_name(domain: PolicyConsumerDomain) -> String {
    format!("{domain:?}").to_lowercase()
}
