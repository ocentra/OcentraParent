use ocentra_network_evidence::apple_network_extension_gate::{
    NetworkAppleNetworkExtensionCapabilityState, NetworkAppleNetworkExtensionGateState,
    NetworkAppleNetworkExtensionPlatform,
};
use ocentra_parent_agent_protocol::network_apple_network_extension_gate_status::{
    NetworkAppleNetworkExtensionGateCapabilityStatusState,
    NetworkAppleNetworkExtensionGateStatusState, NetworkAppleNetworkExtensionPlatformStatus,
};

pub(super) fn protocol_platform(
    platform: NetworkAppleNetworkExtensionPlatform,
) -> NetworkAppleNetworkExtensionPlatformStatus {
    match platform {
        NetworkAppleNetworkExtensionPlatform::MacOs => {
            NetworkAppleNetworkExtensionPlatformStatus::MacOs
        }
        NetworkAppleNetworkExtensionPlatform::Ios => {
            NetworkAppleNetworkExtensionPlatformStatus::Ios
        }
    }
}

pub(super) fn protocol_capability_state(
    state: NetworkAppleNetworkExtensionCapabilityState,
) -> NetworkAppleNetworkExtensionGateCapabilityStatusState {
    match state {
        NetworkAppleNetworkExtensionCapabilityState::AppleDeviceReady => {
            NetworkAppleNetworkExtensionGateCapabilityStatusState::AppleDeviceReady
        }
        NetworkAppleNetworkExtensionCapabilityState::ManualRequired => {
            NetworkAppleNetworkExtensionGateCapabilityStatusState::ManualRequired
        }
        NetworkAppleNetworkExtensionCapabilityState::Unavailable => {
            NetworkAppleNetworkExtensionGateCapabilityStatusState::Unavailable
        }
    }
}

pub(super) fn protocol_gate_state(
    state: NetworkAppleNetworkExtensionGateState,
) -> NetworkAppleNetworkExtensionGateStatusState {
    match state {
        NetworkAppleNetworkExtensionGateState::ResearchOnly => {
            NetworkAppleNetworkExtensionGateStatusState::ResearchOnly
        }
        NetworkAppleNetworkExtensionGateState::ManualRequired => {
            NetworkAppleNetworkExtensionGateStatusState::ManualRequired
        }
        NetworkAppleNetworkExtensionGateState::Unavailable => {
            NetworkAppleNetworkExtensionGateStatusState::Unavailable
        }
        NetworkAppleNetworkExtensionGateState::AppleEntitlementProofReady => {
            NetworkAppleNetworkExtensionGateStatusState::AppleEntitlementProofReady
        }
    }
}
