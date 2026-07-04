use std::time::Duration;

use super::collection::{
    collect_local_neighbor_passive_updates, current_platform_local_neighbor_sources,
    local_neighbor_source_labels, local_neighbor_source_labels_for_platform,
};
use super::{
    LanPassiveDiscoveryListenerState, LanPassiveDiscoveryLocalNeighborCollectionOutcome,
    LanPassiveDiscoveryRawSocketCaptureOutcome, LanPassiveDiscoveryRawSocketProtocol,
    LanPassiveDiscoveryRawSocketSupport,
};

pub fn raw_socket_protocol_support_for_platform(
    protocol: LanPassiveDiscoveryRawSocketProtocol,
    platform: &str,
) -> LanPassiveDiscoveryRawSocketSupport {
    match protocol {
        LanPassiveDiscoveryRawSocketProtocol::Arp => {
            arp_raw_socket_protocol_support(protocol, platform)
        }
        LanPassiveDiscoveryRawSocketProtocol::Dhcp => {
            LanPassiveDiscoveryRawSocketSupport::UnsupportedPlatform {
                protocol,
                platform: platform.to_string(),
                reason: "raw-socket passive capture is not implemented in lan-core".to_string(),
            }
        }
    }
}

pub fn raw_socket_protocol_support(
    protocol: LanPassiveDiscoveryRawSocketProtocol,
) -> LanPassiveDiscoveryRawSocketSupport {
    raw_socket_protocol_support_for_platform(protocol, std::env::consts::OS)
}

pub fn collect_raw_socket_protocol_passive_updates(
    state: &mut LanPassiveDiscoveryListenerState,
    protocol: LanPassiveDiscoveryRawSocketProtocol,
    read_timeout: Duration,
) -> LanPassiveDiscoveryRawSocketCaptureOutcome {
    match protocol {
        LanPassiveDiscoveryRawSocketProtocol::Arp => {
            collect_arp_raw_socket_passive_updates(state, read_timeout)
        }
        LanPassiveDiscoveryRawSocketProtocol::Dhcp => {
            LanPassiveDiscoveryRawSocketCaptureOutcome::Unsupported(raw_socket_protocol_support(
                LanPassiveDiscoveryRawSocketProtocol::Dhcp,
            ))
        }
    }
}

fn arp_raw_socket_protocol_support(
    protocol: LanPassiveDiscoveryRawSocketProtocol,
    platform: &str,
) -> LanPassiveDiscoveryRawSocketSupport {
    let collector_labels = local_neighbor_source_labels_for_platform(platform);
    if collector_labels.is_empty() {
        LanPassiveDiscoveryRawSocketSupport::UnsupportedPlatform {
            protocol,
            platform: platform.to_string(),
            reason: "no passive ARP collector is implemented for this platform".to_string(),
        }
    } else {
        LanPassiveDiscoveryRawSocketSupport::AvailableCollector {
            protocol,
            platform: platform.to_string(),
            collector_labels,
            reason: "lan-core records passive ARP weak hints from OS neighbor tables instead of raw frames"
                .to_string(),
        }
    }
}

fn collect_arp_raw_socket_passive_updates(
    state: &mut LanPassiveDiscoveryListenerState,
    read_timeout: Duration,
) -> LanPassiveDiscoveryRawSocketCaptureOutcome {
    let sources = current_platform_local_neighbor_sources();
    if sources.is_empty() {
        return LanPassiveDiscoveryRawSocketCaptureOutcome::Unsupported(
            raw_socket_protocol_support(LanPassiveDiscoveryRawSocketProtocol::Arp),
        );
    }

    let collector_labels = local_neighbor_source_labels(&sources);
    let mut observed_count = 0_usize;
    let mut recorded_count = 0_usize;
    for source in sources {
        match collect_local_neighbor_passive_updates(state, source, read_timeout) {
            LanPassiveDiscoveryLocalNeighborCollectionOutcome::Captured {
                observed_count: source_observed_count,
                recorded_count: source_recorded_count,
                ..
            } => {
                observed_count += source_observed_count;
                recorded_count += source_recorded_count;
            }
            LanPassiveDiscoveryLocalNeighborCollectionOutcome::Unsupported { reason, .. } => {
                return LanPassiveDiscoveryRawSocketCaptureOutcome::Unsupported(
                    LanPassiveDiscoveryRawSocketSupport::UnsupportedPlatform {
                        protocol: LanPassiveDiscoveryRawSocketProtocol::Arp,
                        platform: std::env::consts::OS.to_string(),
                        reason,
                    },
                );
            }
        }
    }

    LanPassiveDiscoveryRawSocketCaptureOutcome::Captured {
        protocol: LanPassiveDiscoveryRawSocketProtocol::Arp,
        collector_labels,
        observed_count,
        recorded_count,
    }
}
