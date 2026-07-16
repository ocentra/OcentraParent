use std::time::Duration;

use chrono::Utc;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanPassiveDiscoveryLocalNeighborCollectionSummary;

use super::text::compact_summary;
use super::{
    LanPassiveDiscoveryListenerState, LanPassiveDiscoveryLocalNeighborCollectionOutcome,
    LanPassiveDiscoveryLocalNeighborSource, LanPassiveDiscoveryRecordOutcome,
    LanPassiveDiscoverySource, LanPassiveDiscoveryTriggerReason,
};

mod platform;

pub fn local_neighbor_collection_support_for_platform(
    source: LanPassiveDiscoveryLocalNeighborSource,
    platform: &str,
) -> Result<&'static str, String> {
    let source_label = local_neighbor_source_label(&source);
    if local_neighbor_source_supported(source, platform) {
        return Ok(source_label);
    }
    Err(unsupported_local_neighbor_source_reason(
        source,
        source_label,
        platform,
    ))
}

pub fn collect_local_neighbor_passive_updates(
    state: &mut LanPassiveDiscoveryListenerState,
    source: LanPassiveDiscoveryLocalNeighborSource,
    read_timeout: Duration,
) -> LanPassiveDiscoveryLocalNeighborCollectionOutcome {
    let source_label =
        match local_neighbor_collection_support_for_platform(source, std::env::consts::OS) {
            Ok(source_label) => source_label,
            Err(reason) => {
                let source_label = local_neighbor_source_label(&source);
                return LanPassiveDiscoveryLocalNeighborCollectionOutcome::Unsupported {
                    source,
                    source_label,
                    reason,
                };
            }
        };

    let observations = platform::collect_observations(source, read_timeout);
    let (observed_count, recorded_count) =
        record_local_neighbor_passive_updates_from_observations(state, source_label, observations);

    LanPassiveDiscoveryLocalNeighborCollectionOutcome::Captured {
        source,
        source_label,
        observed_count,
        recorded_count,
    }
}

pub fn current_platform_local_neighbor_collection_summaries(
    read_timeout: Duration,
) -> Vec<LanPassiveDiscoveryLocalNeighborCollectionSummary> {
    let mut state = LanPassiveDiscoveryListenerState::running(Utc::now().to_rfc3339());
    current_platform_local_neighbor_sources()
        .into_iter()
        .map(|source| {
            let outcome = collect_local_neighbor_passive_updates(&mut state, source, read_timeout);
            local_neighbor_collection_summary(outcome)
        })
        .collect()
}

pub fn record_local_neighbor_passive_updates_from_observations(
    state: &mut LanPassiveDiscoveryListenerState,
    source_label: &'static str,
    observations: std::collections::HashMap<String, String>,
) -> (usize, usize) {
    let observed_count = observations.len();
    let mut observations = observations.into_iter().collect::<Vec<_>>();
    observations.sort_by(|(left_ip, left_mac), (right_ip, right_mac)| {
        left_ip.cmp(right_ip).then_with(|| left_mac.cmp(right_mac))
    });

    let mut recorded_count = 0_usize;
    for (ip_address, mac_address) in observations {
        let summary = compact_summary(format!(
            "{source_label} weak hint: ip={ip_address}; mac={mac_address}"
        ));
        if let LanPassiveDiscoveryRecordOutcome::Recorded = state.record_passive_update(
            LanPassiveDiscoverySource::Arp,
            LanPassiveDiscoveryTriggerReason::PassivePacketObserved,
            &Utc::now().to_rfc3339(),
            Some(mac_address.as_str()),
            None,
            summary,
        ) {
            recorded_count += 1;
        }
    }

    (observed_count, recorded_count)
}

fn local_neighbor_collection_summary(
    outcome: LanPassiveDiscoveryLocalNeighborCollectionOutcome,
) -> LanPassiveDiscoveryLocalNeighborCollectionSummary {
    match outcome {
        LanPassiveDiscoveryLocalNeighborCollectionOutcome::Captured {
            source_label,
            observed_count,
            recorded_count,
            ..
        } => LanPassiveDiscoveryLocalNeighborCollectionSummary {
            schema_version: LanPassiveDiscoveryListenerState::SCHEMA_VERSION,
            source_label: source_label.to_string(),
            observed_count: observed_count as u32,
            recorded_count: recorded_count as u32,
            reason: None,
        },
        LanPassiveDiscoveryLocalNeighborCollectionOutcome::Unsupported {
            source_label,
            reason,
            ..
        } => LanPassiveDiscoveryLocalNeighborCollectionSummary {
            schema_version: LanPassiveDiscoveryListenerState::SCHEMA_VERSION,
            source_label: source_label.to_string(),
            observed_count: 0,
            recorded_count: 0,
            reason: Some(reason),
        },
    }
}

pub fn current_platform_local_neighbor_sources() -> Vec<LanPassiveDiscoveryLocalNeighborSource> {
    platform::local_neighbor_sources_for_platform(std::env::consts::OS)
}

pub fn local_neighbor_sources_for_platform(
    platform: &str,
) -> Vec<LanPassiveDiscoveryLocalNeighborSource> {
    platform::local_neighbor_sources_for_platform(platform)
}

pub fn local_neighbor_source_labels_for_platform(platform: &str) -> Vec<String> {
    platform::local_neighbor_source_labels_for_platform(platform)
}

pub fn local_neighbor_source_labels(
    sources: &[LanPassiveDiscoveryLocalNeighborSource],
) -> Vec<String> {
    platform::local_neighbor_source_labels(sources)
}

pub fn all_local_neighbor_sources() -> Vec<LanPassiveDiscoveryLocalNeighborSource> {
    platform::all_local_neighbor_sources()
}

pub fn local_neighbor_source_label(
    source: &LanPassiveDiscoveryLocalNeighborSource,
) -> &'static str {
    platform::local_neighbor_source_label(source)
}

fn local_neighbor_source_supported(
    source: LanPassiveDiscoveryLocalNeighborSource,
    platform: &str,
) -> bool {
    platform::local_neighbor_source_supported(source, platform)
}

fn unsupported_local_neighbor_source_reason(
    source: LanPassiveDiscoveryLocalNeighborSource,
    source_label: &str,
    platform: &str,
) -> String {
    platform::unsupported_local_neighbor_source_reason(source, source_label, platform)
}
