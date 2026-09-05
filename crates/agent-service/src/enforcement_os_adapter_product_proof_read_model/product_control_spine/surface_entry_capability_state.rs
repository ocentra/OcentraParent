use ocentra_parent_agent_protocol::enforcement_cross_platform_capability_proof::{
    V08CrossPlatformCapabilityStatus, V08CrossPlatformEnforcementCapabilityClaimState,
};
use ocentra_parent_agent_protocol::enforcement_product_control_spine::{
    V08EnforcementProductControlCapabilityStatus, V08EnforcementProductControlClaimState,
};

pub(super) fn product_capability_status(
    status: V08CrossPlatformCapabilityStatus,
) -> V08EnforcementProductControlCapabilityStatus {
    match status {
        V08CrossPlatformCapabilityStatus::Implemented => {
            V08EnforcementProductControlCapabilityStatus::Implemented
        }
        V08CrossPlatformCapabilityStatus::ManualRequired => {
            V08EnforcementProductControlCapabilityStatus::ManualRequired
        }
        V08CrossPlatformCapabilityStatus::Supported
        | V08CrossPlatformCapabilityStatus::PreviewScaffold
        | V08CrossPlatformCapabilityStatus::Scaffold
        | V08CrossPlatformCapabilityStatus::Unavailable
        | V08CrossPlatformCapabilityStatus::Planned
        | V08CrossPlatformCapabilityStatus::NotImplemented => {
            V08EnforcementProductControlCapabilityStatus::NotImplemented
        }
    }
}

pub(super) fn product_claim_state(
    state: V08CrossPlatformEnforcementCapabilityClaimState,
) -> V08EnforcementProductControlClaimState {
    match state {
        V08CrossPlatformEnforcementCapabilityClaimState::ImplementedBoundary => {
            V08EnforcementProductControlClaimState::ImplementedBoundary
        }
        V08CrossPlatformEnforcementCapabilityClaimState::ManualRequired => {
            V08EnforcementProductControlClaimState::ManualRequired
        }
        V08CrossPlatformEnforcementCapabilityClaimState::Scaffold
        | V08CrossPlatformEnforcementCapabilityClaimState::Unavailable => {
            V08EnforcementProductControlClaimState::Unavailable
        }
        V08CrossPlatformEnforcementCapabilityClaimState::Planned
        | V08CrossPlatformEnforcementCapabilityClaimState::NotClaimed => {
            V08EnforcementProductControlClaimState::NotClaimed
        }
    }
}
