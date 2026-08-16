use ocentra_parent_agent_protocol::enforcement::{
    EnforcementCapabilityStatus, EnforcementUnavailableReason, EnforcementUnavailableStatus,
};

use crate::enforcement_adapter::EnforcementAdapterOutcome;

#[path = "enforcement_unavailable_status_adapter.rs"]
mod enforcement_unavailable_status_adapter;
#[path = "enforcement_unavailable_status_protocol.rs"]
mod enforcement_unavailable_status_protocol;
#[path = "enforcement_unavailable_status_reason.rs"]
mod enforcement_unavailable_status_reason;

pub(super) fn build_unavailable_status(
    schema_version: &str,
    capability: &EnforcementCapabilityStatus,
    unavailable_reason: EnforcementUnavailableReason,
) -> EnforcementUnavailableStatus {
    EnforcementUnavailableStatus {
        schema_version: schema_version.to_string(),
        capability: capability.clone(),
        unavailable_reason,
        retryable: enforcement_unavailable_status_reason::unavailable_reason_is_retryable(
            unavailable_reason,
        ),
        checked_at: capability.last_checked_at.clone(),
    }
}

pub(super) fn capability_unavailable_reason(
    capability: &EnforcementCapabilityStatus,
) -> EnforcementUnavailableReason {
    enforcement_unavailable_status_reason::capability_unavailable_reason(capability)
}

pub(super) fn adapter_unavailable_reason(
    adapter_outcome: &EnforcementAdapterOutcome,
) -> Option<EnforcementUnavailableReason> {
    enforcement_unavailable_status_adapter::adapter_unavailable_reason(adapter_outcome)
}
