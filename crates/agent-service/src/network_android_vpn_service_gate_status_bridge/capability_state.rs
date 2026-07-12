use ocentra_network_evidence::android_vpn_service_gate::{
    NetworkAndroidVpnServiceCapabilityState, NetworkAndroidVpnServiceGateState,
};
use ocentra_parent_agent_protocol::network_android_vpn_service_gate_status::{
    NetworkAndroidVpnServiceGateCapabilityStatusState, NetworkAndroidVpnServiceGateStatusState,
};

pub(super) fn protocol_capability_state(
    state: NetworkAndroidVpnServiceCapabilityState,
) -> NetworkAndroidVpnServiceGateCapabilityStatusState {
    match state {
        NetworkAndroidVpnServiceCapabilityState::PhysicalDeviceReady => {
            NetworkAndroidVpnServiceGateCapabilityStatusState::PhysicalDeviceReady
        }
        NetworkAndroidVpnServiceCapabilityState::ManualRequired => {
            NetworkAndroidVpnServiceGateCapabilityStatusState::ManualRequired
        }
        NetworkAndroidVpnServiceCapabilityState::Unavailable => {
            NetworkAndroidVpnServiceGateCapabilityStatusState::Unavailable
        }
    }
}

pub(super) fn protocol_gate_state(
    state: NetworkAndroidVpnServiceGateState,
) -> NetworkAndroidVpnServiceGateStatusState {
    match state {
        NetworkAndroidVpnServiceGateState::ResearchOnly => {
            NetworkAndroidVpnServiceGateStatusState::ResearchOnly
        }
        NetworkAndroidVpnServiceGateState::ManualRequired => {
            NetworkAndroidVpnServiceGateStatusState::ManualRequired
        }
        NetworkAndroidVpnServiceGateState::Unavailable => {
            NetworkAndroidVpnServiceGateStatusState::Unavailable
        }
        NetworkAndroidVpnServiceGateState::PhysicalDeviceProofReady => {
            NetworkAndroidVpnServiceGateStatusState::PhysicalDeviceProofReady
        }
    }
}
