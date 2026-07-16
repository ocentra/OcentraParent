use crate::{NetworkLinuxAdapterKind, NetworkPlatformClaimTarget};

pub(super) fn linux_target(kind: NetworkLinuxAdapterKind) -> NetworkPlatformClaimTarget {
    match kind {
        NetworkLinuxAdapterKind::Nftables => NetworkPlatformClaimTarget::LinuxNftables,
        NetworkLinuxAdapterKind::Ebpf => NetworkPlatformClaimTarget::LinuxEbpf,
        NetworkLinuxAdapterKind::Tun => NetworkPlatformClaimTarget::LinuxTun,
    }
}
