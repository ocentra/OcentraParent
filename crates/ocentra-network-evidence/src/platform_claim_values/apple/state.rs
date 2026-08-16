use crate::{NetworkAppleNetworkExtensionGateState, NetworkPlatformClaimState};

pub(super) fn apple_state(
    state: NetworkAppleNetworkExtensionGateState,
) -> NetworkPlatformClaimState {
    match state {
        NetworkAppleNetworkExtensionGateState::AppleEntitlementProofReady => {
            NetworkPlatformClaimState::Ready
        }
        NetworkAppleNetworkExtensionGateState::ResearchOnly => {
            NetworkPlatformClaimState::ResearchOnly
        }
        NetworkAppleNetworkExtensionGateState::ManualRequired => {
            NetworkPlatformClaimState::ManualRequired
        }
        NetworkAppleNetworkExtensionGateState::Unavailable => {
            NetworkPlatformClaimState::Unavailable
        }
    }
}
