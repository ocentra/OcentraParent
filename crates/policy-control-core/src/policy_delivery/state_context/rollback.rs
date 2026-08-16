#![forbid(unsafe_code)]

use super::{policy_control, state_values, EventingError, PolicyDeliveryState, RollbackRule};

pub(super) fn validate(
    rollback_reference_state: Option<PolicyDeliveryState>,
    state: PolicyDeliveryState,
    rule: RollbackRule,
) -> Result<(), EventingError> {
    match (rollback_reference_state, rule) {
        (Some(rollback_reference_state), RollbackRule::MustBeAbsent) => {
            Err(EventingError::InvalidValue {
                field: policy_control::delivery::FIELD_ROLLBACK_REFERENCE_STATE,
                value: state_values::unexpected_rollback_reference_state_value(
                    rollback_reference_state,
                    state,
                ),
            })
        }
        (None, RollbackRule::RequiredFrom(_)) => Err(EventingError::InvalidValue {
            field: policy_control::delivery::FIELD_ROLLBACK_REFERENCE_STATE,
            value: state_values::missing_rollback_reference_state_value(state),
        }),
        (Some(rollback_reference_state), RollbackRule::RequiredFrom(allowed_states))
            if !allowed_states.contains(&rollback_reference_state) =>
        {
            Err(EventingError::InvalidValue {
                field: policy_control::delivery::FIELD_ROLLBACK_REFERENCE_STATE,
                value: state_values::policy_delivery_state_name(rollback_reference_state)
                    .to_string(),
            })
        }
        _ => Ok(()),
    }
}
