#![forbid(unsafe_code)]

use super::{
    policy_control, state_values, EventingError, PolicyDeliveryState, PolicyVersion, SupersededRule,
};

pub(super) fn validate(
    superseded_by_policy_version: Option<PolicyVersion>,
    state: PolicyDeliveryState,
    current_policy_version: PolicyVersion,
    rule: SupersededRule,
) -> Result<(), EventingError> {
    match (superseded_by_policy_version, rule) {
        (Some(policy_version), SupersededRule::MustBeAbsent) => Err(EventingError::InvalidValue {
            field: policy_control::delivery::FIELD_SUPERSEDED_BY_POLICY_VERSION,
            value: state_values::unexpected_replacement_policy_version_value(policy_version, state),
        }),
        (None, SupersededRule::RequiredNewerThanCurrent) => Err(EventingError::InvalidValue {
            field: policy_control::delivery::FIELD_SUPERSEDED_BY_POLICY_VERSION,
            value: state_values::missing_replacement_policy_version_value(state),
        }),
        (Some(policy_version), SupersededRule::RequiredNewerThanCurrent)
            if policy_version.value() <= current_policy_version.value() =>
        {
            Err(EventingError::InvalidValue {
                field: policy_control::delivery::FIELD_SUPERSEDED_BY_POLICY_VERSION,
                value: state_values::replacement_policy_version_must_be_newer_value(
                    policy_version,
                    current_policy_version,
                ),
            })
        }
        _ => Ok(()),
    }
}
