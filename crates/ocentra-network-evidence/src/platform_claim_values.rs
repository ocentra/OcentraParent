mod android;
mod apple;
mod linux;
mod refs;
mod windows_firewall;
mod windows_wfp;

use crate::{
    NetworkAndroidVpnServiceGateState, NetworkAndroidVpnServiceRequiredArtifact,
    NetworkAppleNetworkExtensionGateState, NetworkAppleNetworkExtensionPlatform,
    NetworkAppleNetworkExtensionRequiredArtifact, NetworkLinuxAdapterGateState,
    NetworkLinuxAdapterKind, NetworkLinuxAdapterRequiredArtifact, NetworkPlatformClaimState,
    NetworkPlatformClaimTarget, NetworkWindowsFirewallProofState,
    NetworkWindowsFirewallRequiredArtifact, NetworkWindowsWfpGateState,
    NetworkWindowsWfpRequiredArtifact,
};

pub(crate) fn windows_firewall_state(
    state: NetworkWindowsFirewallProofState,
) -> NetworkPlatformClaimState {
    windows_firewall::windows_firewall_state(state)
}

pub(crate) fn windows_wfp_state(state: NetworkWindowsWfpGateState) -> NetworkPlatformClaimState {
    windows_wfp::windows_wfp_state(state)
}

pub(crate) fn android_vpn_state(
    state: NetworkAndroidVpnServiceGateState,
) -> NetworkPlatformClaimState {
    android::android_vpn_state(state)
}

pub(crate) fn apple_state(
    state: NetworkAppleNetworkExtensionGateState,
) -> NetworkPlatformClaimState {
    apple::apple_state(state)
}

pub(crate) fn linux_state(state: NetworkLinuxAdapterGateState) -> NetworkPlatformClaimState {
    linux::linux_state(state)
}

pub(crate) fn apple_target(
    platform: NetworkAppleNetworkExtensionPlatform,
) -> NetworkPlatformClaimTarget {
    apple::apple_target(platform)
}

pub(crate) fn linux_target(kind: NetworkLinuxAdapterKind) -> NetworkPlatformClaimTarget {
    linux::linux_target(kind)
}

pub(crate) fn compact_refs(values: Vec<Option<String>>) -> Vec<String> {
    refs::compact_refs(values)
}

pub(crate) fn windows_firewall_artifact_label(
    artifact: NetworkWindowsFirewallRequiredArtifact,
) -> &'static str {
    windows_firewall::windows_firewall_artifact_label(artifact)
}

pub(crate) fn windows_wfp_artifact_label(
    artifact: NetworkWindowsWfpRequiredArtifact,
) -> &'static str {
    windows_wfp::windows_wfp_artifact_label(artifact)
}

pub(crate) fn android_artifact_label(
    artifact: NetworkAndroidVpnServiceRequiredArtifact,
) -> &'static str {
    android::android_artifact_label(artifact)
}

pub(crate) fn apple_artifact_label(
    artifact: NetworkAppleNetworkExtensionRequiredArtifact,
) -> &'static str {
    apple::apple_artifact_label(artifact)
}

pub(crate) fn linux_artifact_label(artifact: NetworkLinuxAdapterRequiredArtifact) -> &'static str {
    linux::linux_artifact_label(artifact)
}
