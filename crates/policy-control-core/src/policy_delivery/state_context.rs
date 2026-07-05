#![forbid(unsafe_code)]

use super::{
    EventingError, PolicyDeliveryState, PolicyReasonCode, PolicyVersion, policy_control,
    state_values,
};

pub(super) fn assert_state_context(
    state: PolicyDeliveryState,
    reason_code: Option<&PolicyReasonCode>,
    superseded_by_policy_version: Option<PolicyVersion>,
    rollback_reference_state: Option<PolicyDeliveryState>,
    current_policy_version: PolicyVersion,
) -> Result<(), EventingError> {
    match state {
        PolicyDeliveryState::Queued
        | PolicyDeliveryState::Delivering
        | PolicyDeliveryState::Delivered
        | PolicyDeliveryState::Acknowledged
        | PolicyDeliveryState::Applied => assert_clear_state_context(
            reason_code,
            superseded_by_policy_version,
            rollback_reference_state,
            state,
        ),
        PolicyDeliveryState::Rejected
        | PolicyDeliveryState::Degraded
        | PolicyDeliveryState::Offline
        | PolicyDeliveryState::ExpiredBeforeDelivery
        | PolicyDeliveryState::RetryScheduled
        | PolicyDeliveryState::PartialDomainApply
        | PolicyDeliveryState::BlockedByPermission
        | PolicyDeliveryState::BlockedByCapability
        | PolicyDeliveryState::ManualRequired => assert_reason_required_state_context(
            reason_code,
            superseded_by_policy_version,
            rollback_reference_state,
            state,
        ),
        PolicyDeliveryState::Superseded => assert_superseded_state_context(
            reason_code,
            superseded_by_policy_version,
            rollback_reference_state,
            current_policy_version,
            state,
        ),
        PolicyDeliveryState::RolledBack => assert_rolled_back_state_context(
            reason_code,
            superseded_by_policy_version,
            rollback_reference_state,
            state,
        ),
    }
}

fn assert_clear_state_context(
    reason_code: Option<&PolicyReasonCode>,
    superseded_by_policy_version: Option<PolicyVersion>,
    rollback_reference_state: Option<PolicyDeliveryState>,
    state: PolicyDeliveryState,
) -> Result<(), EventingError> {
    if let Some(reason_code) = reason_code {
        return Err(EventingError::InvalidValue {
            field: policy_control::delivery::FIELD_REASON_CODE,
            value: state_values::unexpected_reason_code_value(reason_code, state),
        });
    }
    if let Some(superseded_by_policy_version) = superseded_by_policy_version {
        return Err(EventingError::InvalidValue {
            field: policy_control::delivery::FIELD_SUPERSEDED_BY_POLICY_VERSION,
            value: state_values::unexpected_replacement_policy_version_value(
                superseded_by_policy_version,
                state,
            ),
        });
    }
    if let Some(rollback_reference_state) = rollback_reference_state {
        return Err(EventingError::InvalidValue {
            field: policy_control::delivery::FIELD_ROLLBACK_REFERENCE_STATE,
            value: state_values::unexpected_rollback_reference_state_value(
                rollback_reference_state,
                state,
            ),
        });
    }

    Ok(())
}

fn assert_reason_required_state_context(
    reason_code: Option<&PolicyReasonCode>,
    superseded_by_policy_version: Option<PolicyVersion>,
    rollback_reference_state: Option<PolicyDeliveryState>,
    state: PolicyDeliveryState,
) -> Result<(), EventingError> {
    if reason_code.is_none() {
        return Err(EventingError::InvalidValue {
            field: policy_control::delivery::FIELD_REASON_CODE,
            value: state_values::missing_reason_code_value(state),
        });
    }
    if let Some(superseded_by_policy_version) = superseded_by_policy_version {
        return Err(EventingError::InvalidValue {
            field: policy_control::delivery::FIELD_SUPERSEDED_BY_POLICY_VERSION,
            value: state_values::unexpected_replacement_policy_version_value(
                superseded_by_policy_version,
                state,
            ),
        });
    }
    if let Some(rollback_reference_state) = rollback_reference_state {
        return Err(EventingError::InvalidValue {
            field: policy_control::delivery::FIELD_ROLLBACK_REFERENCE_STATE,
            value: state_values::unexpected_rollback_reference_state_value(
                rollback_reference_state,
                state,
            ),
        });
    }

    Ok(())
}

fn assert_superseded_state_context(
    reason_code: Option<&PolicyReasonCode>,
    superseded_by_policy_version: Option<PolicyVersion>,
    rollback_reference_state: Option<PolicyDeliveryState>,
    current_policy_version: PolicyVersion,
    state: PolicyDeliveryState,
) -> Result<(), EventingError> {
    if let Some(reason_code) = reason_code {
        return Err(EventingError::InvalidValue {
            field: policy_control::delivery::FIELD_REASON_CODE,
            value: state_values::unexpected_reason_code_value(reason_code, state),
        });
    }
    if let Some(rollback_reference_state) = rollback_reference_state {
        return Err(EventingError::InvalidValue {
            field: policy_control::delivery::FIELD_ROLLBACK_REFERENCE_STATE,
            value: state_values::unexpected_rollback_reference_state_value(
                rollback_reference_state,
                state,
            ),
        });
    }

    let superseded_by_policy_version =
        superseded_by_policy_version.ok_or_else(|| EventingError::InvalidValue {
            field: policy_control::delivery::FIELD_SUPERSEDED_BY_POLICY_VERSION,
            value: state_values::missing_replacement_policy_version_value(state),
        })?;

    if superseded_by_policy_version.value() <= current_policy_version.value() {
        return Err(EventingError::InvalidValue {
            field: policy_control::delivery::FIELD_SUPERSEDED_BY_POLICY_VERSION,
            value: state_values::replacement_policy_version_must_be_newer_value(
                superseded_by_policy_version,
                current_policy_version,
            ),
        });
    }

    Ok(())
}

fn assert_rolled_back_state_context(
    reason_code: Option<&PolicyReasonCode>,
    superseded_by_policy_version: Option<PolicyVersion>,
    rollback_reference_state: Option<PolicyDeliveryState>,
    state: PolicyDeliveryState,
) -> Result<(), EventingError> {
    if reason_code.is_none() {
        return Err(EventingError::InvalidValue {
            field: policy_control::delivery::FIELD_REASON_CODE,
            value: state_values::missing_reason_code_value(state),
        });
    }
    if let Some(superseded_by_policy_version) = superseded_by_policy_version {
        return Err(EventingError::InvalidValue {
            field: policy_control::delivery::FIELD_SUPERSEDED_BY_POLICY_VERSION,
            value: state_values::unexpected_replacement_policy_version_value(
                superseded_by_policy_version,
                state,
            ),
        });
    }

    let rollback_reference_state =
        rollback_reference_state.ok_or_else(|| EventingError::InvalidValue {
            field: policy_control::delivery::FIELD_ROLLBACK_REFERENCE_STATE,
            value: state_values::missing_rollback_reference_state_value(state),
        })?;

    if !matches!(
        rollback_reference_state,
        PolicyDeliveryState::Delivered
            | PolicyDeliveryState::Acknowledged
            | PolicyDeliveryState::Applied
            | PolicyDeliveryState::PartialDomainApply
            | PolicyDeliveryState::Degraded
            | PolicyDeliveryState::Offline
    ) {
        return Err(EventingError::InvalidValue {
            field: policy_control::delivery::FIELD_ROLLBACK_REFERENCE_STATE,
            value: state_values::policy_delivery_state_name(rollback_reference_state).to_string(),
        });
    }

    Ok(())
}
