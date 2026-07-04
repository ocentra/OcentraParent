mod labels;
mod state;
mod target;

use crate::{
    NetworkAppleNetworkExtensionGateState, NetworkAppleNetworkExtensionPlatform,
    NetworkAppleNetworkExtensionRequiredArtifact, NetworkPlatformClaimState,
    NetworkPlatformClaimTarget,
};

pub(super) fn apple_state(
    state: NetworkAppleNetworkExtensionGateState,
) -> NetworkPlatformClaimState {
    state::apple_state(state)
}

pub(super) fn apple_target(
    platform: NetworkAppleNetworkExtensionPlatform,
) -> NetworkPlatformClaimTarget {
    target::apple_target(platform)
}

pub(super) fn apple_artifact_label(
    artifact: NetworkAppleNetworkExtensionRequiredArtifact,
) -> &'static str {
    labels::apple_artifact_label(artifact)
}
