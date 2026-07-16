mod labels;
mod state;

use crate::{
    NetworkPlatformClaimState, NetworkWindowsWfpGateState, NetworkWindowsWfpRequiredArtifact,
};

pub(super) fn windows_wfp_state(state: NetworkWindowsWfpGateState) -> NetworkPlatformClaimState {
    state::windows_wfp_state(state)
}

pub(super) fn windows_wfp_artifact_label(
    artifact: NetworkWindowsWfpRequiredArtifact,
) -> &'static str {
    labels::windows_wfp_artifact_label(artifact)
}
