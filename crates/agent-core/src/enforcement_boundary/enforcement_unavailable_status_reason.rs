use ocentra_parent_agent_protocol::constants::enforcement as enforcement_constants;
use ocentra_parent_agent_protocol::enforcement::{
    EnforcementCapabilityStatus, EnforcementDependencyState, EnforcementPermissionState,
    EnforcementUnavailableReason,
};

pub(super) fn capability_unavailable_reason(
    capability: &EnforcementCapabilityStatus,
) -> EnforcementUnavailableReason {
    if capability.permission_state == EnforcementPermissionState::MissingPermission {
        return EnforcementUnavailableReason::MissingPermission;
    }
    if capability.dependency_state == EnforcementDependencyState::Missing {
        return EnforcementUnavailableReason::MissingDependency;
    }
    if capability.degraded_reason.as_deref()
        == Some(enforcement_constants::UNAVAILABLE_UNSUPPORTED_PLATFORM)
    {
        return EnforcementUnavailableReason::UnsupportedPlatform;
    }
    if capability.degraded_reason.as_deref()
        == Some(enforcement_constants::UNAVAILABLE_MANUAL_REQUIRED)
    {
        return EnforcementUnavailableReason::ManualRequired;
    }

    EnforcementUnavailableReason::AdapterUnavailable
}

pub(super) fn unavailable_reason_is_retryable(
    unavailable_reason: EnforcementUnavailableReason,
) -> bool {
    matches!(
        unavailable_reason,
        EnforcementUnavailableReason::AdapterUnavailable
            | EnforcementUnavailableReason::AdapterError
    )
}
