mod labels;
mod state;

use crate::{
    NetworkAndroidVpnServiceGateState, NetworkAndroidVpnServiceRequiredArtifact,
    NetworkPlatformClaimState,
};

pub(super) fn android_vpn_state(
    state: NetworkAndroidVpnServiceGateState,
) -> NetworkPlatformClaimState {
    self::state::android_vpn_state(state)
}

pub(super) fn android_artifact_label(
    artifact: NetworkAndroidVpnServiceRequiredArtifact,
) -> &'static str {
    labels::android_artifact_label(artifact)
}
