use super::super::{
    LanPassiveDiscoveryListenerState, LanPassiveDiscoveryPacketIngestOutcome,
    LanPassiveDiscoveryRecordOutcome,
};

pub(super) fn ingest_udp_packet(
    state: &mut LanPassiveDiscoveryListenerState,
    payload: &[u8],
) -> LanPassiveDiscoveryPacketIngestOutcome {
    if !state.is_running() {
        return LanPassiveDiscoveryPacketIngestOutcome::Stopped;
    }
    match super::super::packet::parse_passive_discovery_packet(payload) {
        Ok(packet) => record_packet(state, packet),
        Err(error) => LanPassiveDiscoveryPacketIngestOutcome::Rejected(error),
    }
}

fn record_packet(
    state: &mut LanPassiveDiscoveryListenerState,
    packet: super::super::LanPassiveDiscoveryPacket,
) -> LanPassiveDiscoveryPacketIngestOutcome {
    match state.record_passive_packet(packet) {
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
