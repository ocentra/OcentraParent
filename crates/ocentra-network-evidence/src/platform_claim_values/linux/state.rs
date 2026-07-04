use crate::{NetworkLinuxAdapterGateState, NetworkPlatformClaimState};

pub(super) fn linux_state(state: NetworkLinuxAdapterGateState) -> NetworkPlatformClaimState {
    match state {
        NetworkLinuxAdapterGateState::DistroProofReady => NetworkPlatformClaimState::Ready,
        NetworkLinuxAdapterGateState::ResearchOnly => NetworkPlatformClaimState::ResearchOnly,
        NetworkLinuxAdapterGateState::ManualRequired => NetworkPlatformClaimState::ManualRequired,
        NetworkLinuxAdapterGateState::Unavailable => NetworkPlatformClaimState::Unavailable,
    }
}
