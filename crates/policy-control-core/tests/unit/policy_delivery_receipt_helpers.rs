use super::policy_delivery_helpers as helpers;
use super::TestResult;
use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::constants::policy_control;
use ocentra_policy_control_core::policy_delivery::{
    PolicyDeliveryExecutionReceipt, PolicyDeliveryRecord, PolicyDeliveryState,
    PolicyDeliveryTransition,
};

pub(super) fn delivery_state_name(state: PolicyDeliveryState) -> String {
    test_ok!(
        serde_json::to_string(&state),
        "serialize delivery state for assertions"
    )
    .trim_matches('"')
    .to_string()
}

pub(super) fn execution_receipt_with_sequence(
    current: &PolicyDeliveryRecord,
    transition: &PolicyDeliveryTransition,
    sequence: u64,
) -> PolicyDeliveryExecutionReceipt {
    PolicyDeliveryExecutionReceipt {
        sequence: test_ok!(
            ocentra_policy_control_core::policy_delivery::PolicyDeliverySequence::new(sequence),
            "policy delivery receipt sequence"
        ),
        ..helpers::execution_receipt(current, transition)
    }
}

pub(super) fn assert_unexpected_adapter_execution_receipt(
    current: &PolicyDeliveryRecord,
    state: PolicyDeliveryState,
) -> TestResult {
    let transition = helpers::transition(2, "attempt-unexpected-receipt", state)?;
    let receipt = helpers::execution_receipt(current, &transition);

    let error = test_err!(
        ocentra_policy_control_core::policy_delivery::validate_policy_delivery_execution_receipt(
            current,
            &transition,
            Some(&receipt),
        ),
        "unexpected adapter execution receipt must fail"
    );
    assert_eq!(
        error,
        EventingError::InvalidValue {
            field: policy_control::delivery::FIELD_STATE,
            value: format!(
                "unexpected adapter execution receipt for {} with receipt sequence 2",
                delivery_state_name(state)
            ),
        }
    );
    Ok(())
}
