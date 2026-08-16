use crate::{NetworkAndroidVpnServiceGateState, NetworkPlatformClaimState};

pub(super) fn android_vpn_state(
    state: NetworkAndroidVpnServiceGateState,
) -> NetworkPlatformClaimState {
    match state {
        NetworkAndroidVpnServiceGateState::PhysicalDeviceProofReady => {
            NetworkPlatformClaimState::Ready
        }
        NetworkAndroidVpnServiceGateState::ResearchOnly => NetworkPlatformClaimState::ResearchOnly,
        NetworkAndroidVpnServiceGateState::ManualRequired => {
            NetworkPlatformClaimState::ManualRequired
        }
        NetworkAndroidVpnServiceGateState::Unavailable => NetworkPlatformClaimState::Unavailable,
    }
}
