#![forbid(unsafe_code)]

use super::{
    EventingError, PolicyDeliveryExecutionReceipt, PolicyDeliveryRecord, PolicyDeliveryTransition,
};
use crate::policy_delivery::{policy_control, state_values};
use crate::policy_source::PolicyConsumerDomain;

const FIELD_TARGET_DOMAIN: &str = "policy_delivery.target.domain";
const CURRENT_RECORD_IDENTITY_MISMATCH: &str =
    "execution receipt identity mismatch: expected=current-record, reported=execution-receipt";
const TRANSITION_IDENTITY_MISMATCH: &str =
    "execution receipt identity mismatch: expected=transition, reported=execution-receipt";

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
            CURRENT_RECORD_IDENTITY_MISMATCH.to_string(),
        ),
        (
            receipt.household_id != current.household_id,
            policy_control::source::FIELD_HOUSEHOLD_ID,
            CURRENT_RECORD_IDENTITY_MISMATCH.to_string(),
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
            receipt.source_document_id != current.source_document_id,
            policy_control::source::FIELD_DOCUMENT_ID,
            CURRENT_RECORD_IDENTITY_MISMATCH.to_string(),
        ),
        (
            receipt.target.child_profile_id != current.target.child_profile_id,
            policy_control::source::FIELD_CHILD_PROFILE_ID,
            CURRENT_RECORD_IDENTITY_MISMATCH.to_string(),
        ),
        (
            receipt.target.device_id != current.target.device_id,
            policy_control::source::FIELD_DEVICE_ID,
            CURRENT_RECORD_IDENTITY_MISMATCH.to_string(),
        ),
        (
            receipt.target.domain != current.target.domain,
            FIELD_TARGET_DOMAIN,
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
            TRANSITION_IDENTITY_MISMATCH.to_string(),
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
        (
            receipt.reason_code != transition.reason_code,
            policy_control::delivery::FIELD_REASON_CODE,
            TRANSITION_IDENTITY_MISMATCH.to_string(),
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
    match domain {
        PolicyConsumerDomain::App => String::from("app"),
        PolicyConsumerDomain::Browser => String::from("browser"),
        PolicyConsumerDomain::Network => String::from("network"),
        PolicyConsumerDomain::Tracking => String::from("tracking"),
        PolicyConsumerDomain::Screen => String::from("screen"),
        PolicyConsumerDomain::Ai => String::from("ai"),
    }
}
