use std::time::Duration;

use super::super::collection::{
    collect_local_neighbor_passive_updates, current_platform_local_neighbor_sources,
    local_neighbor_source_labels, local_neighbor_source_labels_for_platform,
};
use super::super::{
    LanPassiveDiscoveryListenerState, LanPassiveDiscoveryLocalNeighborCollectionOutcome,
    LanPassiveDiscoveryRawSocketCaptureOutcome, LanPassiveDiscoveryRawSocketProtocol,
    LanPassiveDiscoveryRawSocketSupport,
};

pub(super) fn protocol_support(
    protocol: LanPassiveDiscoveryRawSocketProtocol,
    platform: &str,
) -> LanPassiveDiscoveryRawSocketSupport {
    let collector_labels = local_neighbor_source_labels_for_platform(platform);
    if collector_labels.is_empty() {
        return LanPassiveDiscoveryRawSocketSupport::UnsupportedPlatform {
            protocol,
            platform: platform.to_string(),
            reason: "no passive ARP collector is implemented for this platform".to_string(),
        };
    }
    LanPassiveDiscoveryRawSocketSupport::AvailableCollector {
        protocol,
        platform: platform.to_string(),
        collector_labels,
        reason:
            "lan-core records passive ARP weak hints from OS neighbor tables instead of raw frames"
                .to_string(),
    }
}

pub(super) fn collect_passive_updates(
    state: &mut LanPassiveDiscoveryListenerState,
    read_timeout: Duration,
) -> LanPassiveDiscoveryRawSocketCaptureOutcome {
    let sources = current_platform_local_neighbor_sources();
    if sources.is_empty() {
        return unsupported_arp_capture();
    }

    let collector_labels = local_neighbor_source_labels(&sources);
    let (observed_count, recorded_count) =
        match collect_source_updates(state, sources, read_timeout) {
            Ok(counts) => counts,
            Err(reason) => return unsupported_arp_capture_with_reason(reason),
        };
    LanPassiveDiscoveryRawSocketCaptureOutcome::Captured {
        protocol: LanPassiveDiscoveryRawSocketProtocol::Arp,
        collector_labels,
        observed_count,
        recorded_count,
    }
}

fn collect_source_updates(
    state: &mut LanPassiveDiscoveryListenerState,
    sources: Vec<super::super::LanPassiveDiscoveryLocalNeighborSource>,
    read_timeout: Duration,
) -> Result<(usize, usize), String> {
    sources
        .into_iter()
        .try_fold(
            (0_usize, 0_usize),
            |counts, source| match collect_local_neighbor_passive_updates(
                state,
                source,
                read_timeout,
            ) {
                LanPassiveDiscoveryLocalNeighborCollectionOutcome::Captured {
                    observed_count,
                    recorded_count,
                    ..
                } => Ok((counts.0 + observed_count, counts.1 + recorded_count)),
                LanPassiveDiscoveryLocalNeighborCollectionOutcome::Unsupported {
                    reason, ..
                } => Err(reason),
            },
        )
}

fn unsupported_arp_capture() -> LanPassiveDiscoveryRawSocketCaptureOutcome {
    LanPassiveDiscoveryRawSocketCaptureOutcome::Unsupported(protocol_support(
        LanPassiveDiscoveryRawSocketProtocol::Arp,
        std::env::consts::OS,
    ))
}

fn unsupported_arp_capture_with_reason(
    reason: String,
) -> LanPassiveDiscoveryRawSocketCaptureOutcome {
    LanPassiveDiscoveryRawSocketCaptureOutcome::Unsupported(
        LanPassiveDiscoveryRawSocketSupport::UnsupportedPlatform {
            protocol: LanPassiveDiscoveryRawSocketProtocol::Arp,
            platform: std::env::consts::OS.to_string(),
            reason,
        },
    )
}
