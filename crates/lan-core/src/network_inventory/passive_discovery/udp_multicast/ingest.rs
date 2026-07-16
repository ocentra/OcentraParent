use std::net::UdpSocket;

use chrono::Utc;

use super::super::summaries::{passive_native_datagram_device_id, passive_native_datagram_summary};
use super::super::{
    LanPassiveDiscoveryListenerState, LanPassiveDiscoveryPacketIngestOutcome,
    LanPassiveDiscoveryPacketParseError, LanPassiveDiscoveryRecordOutcome,
    LanPassiveDiscoverySource, LanPassiveDiscoveryTriggerReason,
};

mod drain;

pub(super) fn drain_udp_socket_packets(
    socket: &UdpSocket,
    state: &mut LanPassiveDiscoveryListenerState,
    source: LanPassiveDiscoverySource,
    max_datagram_count: usize,
) -> usize {
    drain::drain_udp_socket_packets_with_observed_at(
        socket,
        state,
        source,
        max_datagram_count,
        &mut || Utc::now().to_rfc3339(),
    )
}

pub(super) fn drain_udp_socket_packets_with_observed_at(
    socket: &UdpSocket,
    state: &mut LanPassiveDiscoveryListenerState,
    source: LanPassiveDiscoverySource,
    max_datagram_count: usize,
    observed_at: &mut dyn FnMut() -> String,
) -> usize {
    drain::drain_udp_socket_packets_with_observed_at(
        socket,
        state,
        source,
        max_datagram_count,
        observed_at,
    )
}

pub(super) fn ingest_passive_datagram(
    state: &mut LanPassiveDiscoveryListenerState,
    source: &LanPassiveDiscoverySource,
    payload: &[u8],
) -> LanPassiveDiscoveryPacketIngestOutcome {
    let observed_at = Utc::now().to_rfc3339();
    ingest_passive_datagram_with_observed_at(state, source, payload, &observed_at)
}

pub(super) fn ingest_passive_datagram_with_observed_at(
    state: &mut LanPassiveDiscoveryListenerState,
    source: &LanPassiveDiscoverySource,
    payload: &[u8],
    observed_at: &str,
) -> LanPassiveDiscoveryPacketIngestOutcome {
    match state.ingest_udp_packet(payload) {
        LanPassiveDiscoveryPacketIngestOutcome::Rejected(
            LanPassiveDiscoveryPacketParseError::MalformedPayload,
        ) => {}
        outcome => return outcome,
    }
    let Some(summary) = passive_native_datagram_summary(*source, payload) else {
        return LanPassiveDiscoveryPacketIngestOutcome::Rejected(
            LanPassiveDiscoveryPacketParseError::MalformedPayload,
        );
    };
    let device_id = passive_native_datagram_device_id(*source, payload);
    match state.record_passive_update(
        *source,
        LanPassiveDiscoveryTriggerReason::PassivePacketObserved,
        observed_at,
        device_id.as_deref(),
        None,
        summary,
    ) {
        LanPassiveDiscoveryRecordOutcome::Recorded => {
            LanPassiveDiscoveryPacketIngestOutcome::Recorded
        }
        LanPassiveDiscoveryRecordOutcome::Deduplicated => {
            LanPassiveDiscoveryPacketIngestOutcome::Deduplicated
        }
        LanPassiveDiscoveryRecordOutcome::Stopped => {
            LanPassiveDiscoveryPacketIngestOutcome::Stopped
        }
    }
}
