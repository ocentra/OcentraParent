use ocentra_parent_agent_protocol::constants::enforcement as enforcement_constants;
use ocentra_parent_agent_protocol::enforcement::{
    EnforcementAdapterResultCode, EnforcementCapabilityStatus, EnforcementDependencyState,
    EnforcementPermissionState, EnforcementResultStatus, EnforcementUnavailableReason,
    EnforcementUnavailableStatus,
};

use crate::enforcement_adapter::EnforcementAdapterOutcome;

pub(crate) fn build_unavailable_status(
    schema_version: &str,
    capability: &EnforcementCapabilityStatus,
    unavailable_reason: EnforcementUnavailableReason,
) -> EnforcementUnavailableStatus {
    EnforcementUnavailableStatus {
        schema_version: schema_version.to_string(),
        capability: capability.clone(),
        unavailable_reason,
        retryable: unavailable_reason_is_retryable(unavailable_reason),
        checked_at: capability.last_checked_at.clone(),
    }
}

pub(crate) fn capability_unavailable_reason(
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

pub(crate) fn adapter_unavailable_reason(
    adapter_outcome: &EnforcementAdapterOutcome,
) -> Option<EnforcementUnavailableReason> {
    if adapter_outcome.status != EnforcementResultStatus::Unavailable {
        return None;
    }

    adapter_outcome
        .unavailable_reason
        .as_deref()
        .and_then(unavailable_reason_from_protocol_str)
        .or(Some(match adapter_outcome.adapter_result_code {
            EnforcementAdapterResultCode::UnsupportedPlatform => {
                EnforcementUnavailableReason::UnsupportedPlatform
            }
            EnforcementAdapterResultCode::AdapterFailed => {
                EnforcementUnavailableReason::AdapterError
            }
            _ => EnforcementUnavailableReason::AdapterUnavailable,
        }))
}

fn unavailable_reason_from_protocol_str(reason: &str) -> Option<EnforcementUnavailableReason> {
    match reason {
        enforcement_constants::UNAVAILABLE_UNSUPPORTED_PLATFORM => {
            Some(EnforcementUnavailableReason::UnsupportedPlatform)
        }
        enforcement_constants::UNAVAILABLE_UNSUPPORTED_ACTION => {
            Some(EnforcementUnavailableReason::UnsupportedAction)
        }
        enforcement_constants::UNAVAILABLE_MISSING_PERMISSION => {
            Some(EnforcementUnavailableReason::MissingPermission)
        }
        enforcement_constants::UNAVAILABLE_MISSING_DEPENDENCY => {
            Some(EnforcementUnavailableReason::MissingDependency)
        }
        enforcement_constants::UNAVAILABLE_ADAPTER_UNAVAILABLE => {
            Some(EnforcementUnavailableReason::AdapterUnavailable)
        }
        enforcement_constants::UNAVAILABLE_ADAPTER_ERROR => {
            Some(EnforcementUnavailableReason::AdapterError)
        }
        enforcement_constants::UNAVAILABLE_MANUAL_REQUIRED => {
            Some(EnforcementUnavailableReason::ManualRequired)
        }
        _ => None,
    }
}

fn unavailable_reason_is_retryable(unavailable_reason: EnforcementUnavailableReason) -> bool {
    matches!(
        unavailable_reason,
        EnforcementUnavailableReason::AdapterUnavailable
            | EnforcementUnavailableReason::AdapterError
    )
}
