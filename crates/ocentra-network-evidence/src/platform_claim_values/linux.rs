mod labels;
mod state;
mod target;

use crate::{
    NetworkLinuxAdapterGateState, NetworkLinuxAdapterKind, NetworkLinuxAdapterRequiredArtifact,
    NetworkPlatformClaimState, NetworkPlatformClaimTarget,
};

pub(super) fn linux_state(state: NetworkLinuxAdapterGateState) -> NetworkPlatformClaimState {
    state::linux_state(state)
}

pub(super) fn linux_target(kind: NetworkLinuxAdapterKind) -> NetworkPlatformClaimTarget {
    target::linux_target(kind)
}

pub(super) fn linux_artifact_label(artifact: NetworkLinuxAdapterRequiredArtifact) -> &'static str {
    labels::linux_artifact_label(artifact)
}
