use ocentra_parent_agent_protocol::constants::enforcement as enforcement_constants;
use ocentra_parent_agent_protocol::enforcement::EnforcementUnavailableReason;

pub(super) fn unavailable_reason_from_protocol_str(
    reason: &str,
) -> Option<EnforcementUnavailableReason> {
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
