use crate::{NetworkPlatformClaimState, NetworkWindowsWfpGateState};

pub(super) fn windows_wfp_state(state: NetworkWindowsWfpGateState) -> NetworkPlatformClaimState {
    match state {
        NetworkWindowsWfpGateState::LabProofReady => NetworkPlatformClaimState::Ready,
        NetworkWindowsWfpGateState::ResearchOnly => NetworkPlatformClaimState::ResearchOnly,
        NetworkWindowsWfpGateState::ManualRequired => NetworkPlatformClaimState::ManualRequired,
        NetworkWindowsWfpGateState::Unavailable => NetworkPlatformClaimState::Unavailable,
    }
}
