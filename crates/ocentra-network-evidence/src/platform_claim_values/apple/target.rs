use crate::{NetworkAppleNetworkExtensionPlatform, NetworkPlatformClaimTarget};

pub(super) fn apple_target(
    platform: NetworkAppleNetworkExtensionPlatform,
) -> NetworkPlatformClaimTarget {
    match platform {
        NetworkAppleNetworkExtensionPlatform::MacOs => {
            NetworkPlatformClaimTarget::AppleNetworkExtensionMacOs
        }
        NetworkAppleNetworkExtensionPlatform::Ios => {
            NetworkPlatformClaimTarget::AppleNetworkExtensionIos
        }
    }
}
