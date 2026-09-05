use ocentra_parent_agent_protocol::enforcement_cross_platform_capability_proof::{
    V08CrossPlatformAdapterExecutionState, V08CrossPlatformEnforcementCapabilityClaimState,
};
use ocentra_parent_agent_protocol::enforcement_product_control_spine::{
    V08EnforcementProductControlDevicePolicyState, V08EnforcementProductControlExecutionState,
};

pub(super) fn product_execution_state(
    state: V08CrossPlatformAdapterExecutionState,
) -> V08EnforcementProductControlExecutionState {
    match state {
        V08CrossPlatformAdapterExecutionState::ExecutesRealService => {
            V08EnforcementProductControlExecutionState::ExecutesRealService
        }
        V08CrossPlatformAdapterExecutionState::ReturnsManualRequired => {
            V08EnforcementProductControlExecutionState::ReturnsManualRequired
        }
        V08CrossPlatformAdapterExecutionState::ReturnsUnavailable => {
            V08EnforcementProductControlExecutionState::ReturnsUnavailable
        }
        V08CrossPlatformAdapterExecutionState::ScaffoldOnly
        | V08CrossPlatformAdapterExecutionState::NotInvoked => {
            V08EnforcementProductControlExecutionState::NotInvoked
        }
    }
}

pub(super) fn product_device_policy_state(
    state: V08CrossPlatformEnforcementCapabilityClaimState,
) -> V08EnforcementProductControlDevicePolicyState {
    match state {
        V08CrossPlatformEnforcementCapabilityClaimState::ImplementedBoundary => {
            V08EnforcementProductControlDevicePolicyState::ControlCapable
        }
        V08CrossPlatformEnforcementCapabilityClaimState::ManualRequired => {
            V08EnforcementProductControlDevicePolicyState::ManualRequired
        }
        V08CrossPlatformEnforcementCapabilityClaimState::Scaffold
        | V08CrossPlatformEnforcementCapabilityClaimState::Unavailable => {
            V08EnforcementProductControlDevicePolicyState::Unavailable
        }
        V08CrossPlatformEnforcementCapabilityClaimState::Planned
        | V08CrossPlatformEnforcementCapabilityClaimState::NotClaimed => {
            V08EnforcementProductControlDevicePolicyState::NotClaimed
        }
    }
}
