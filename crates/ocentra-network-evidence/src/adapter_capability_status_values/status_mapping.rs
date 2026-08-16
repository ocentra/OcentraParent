use crate::adapter_capability_status::NetworkAdapterCapabilityStatusState;
use crate::platform_claims::{NetworkPlatformClaimState, NetworkPlatformClaimTarget};

pub(crate) fn status_from_platform_entry(
    target: NetworkPlatformClaimTarget,
    state: NetworkPlatformClaimState,
) -> NetworkAdapterCapabilityStatusState {
    match state {
        NetworkPlatformClaimState::Ready => ready_status_for_target(target),
        NetworkPlatformClaimState::DryRun => NetworkAdapterCapabilityStatusState::DryRun,
        NetworkPlatformClaimState::ResearchOnly => {
            NetworkAdapterCapabilityStatusState::ResearchOnly
        }
        NetworkPlatformClaimState::ManualRequired => {
            NetworkAdapterCapabilityStatusState::ManualRequired
        }
        NetworkPlatformClaimState::Unavailable => NetworkAdapterCapabilityStatusState::Unavailable,
    }
}

fn ready_status_for_target(
    target: NetworkPlatformClaimTarget,
) -> NetworkAdapterCapabilityStatusState {
    match target {
        NetworkPlatformClaimTarget::WindowsFirewall => {
            NetworkAdapterCapabilityStatusState::Supported
        }
        NetworkPlatformClaimTarget::WindowsWfp => NetworkAdapterCapabilityStatusState::LabReady,
        NetworkPlatformClaimTarget::AndroidVpnService => {
            NetworkAdapterCapabilityStatusState::PhysicalDeviceReady
        }
        NetworkPlatformClaimTarget::AppleNetworkExtensionMacOs
        | NetworkPlatformClaimTarget::AppleNetworkExtensionIos => {
            NetworkAdapterCapabilityStatusState::AppleDeviceReady
        }
        NetworkPlatformClaimTarget::LinuxNftables
        | NetworkPlatformClaimTarget::LinuxEbpf
        | NetworkPlatformClaimTarget::LinuxTun => NetworkAdapterCapabilityStatusState::DistroReady,
    }
}
