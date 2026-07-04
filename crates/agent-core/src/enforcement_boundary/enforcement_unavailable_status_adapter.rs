use ocentra_parent_agent_protocol::enforcement::{
    EnforcementAdapterResultCode, EnforcementResultStatus, EnforcementUnavailableReason,
};

use crate::enforcement_adapter::EnforcementAdapterOutcome;

use super::enforcement_unavailable_status_protocol::unavailable_reason_from_protocol_str;

pub(super) fn adapter_unavailable_reason(
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
