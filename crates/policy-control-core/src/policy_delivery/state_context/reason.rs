#![forbid(unsafe_code)]

use super::{
    policy_control, state_values, EventingError, PolicyDeliveryState, PolicyReasonCode, ReasonRule,
};

pub(super) fn validate(
    reason_code: Option<&PolicyReasonCode>,
    state: PolicyDeliveryState,
    rule: ReasonRule,
) -> Result<(), EventingError> {
    match (reason_code, rule) {
        (Some(reason_code), ReasonRule::MustBeAbsent) => Err(EventingError::InvalidValue {
            field: policy_control::delivery::FIELD_REASON_CODE,
            value: state_values::unexpected_reason_code_value(reason_code, state),
        }),
        (None, ReasonRule::Required) => Err(EventingError::InvalidValue {
            field: policy_control::delivery::FIELD_REASON_CODE,
            value: state_values::missing_reason_code_value(state),
        }),
        _ => Ok(()),
    }
}
