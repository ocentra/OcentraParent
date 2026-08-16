use serde::{Deserialize, Serialize};

use super::{
    LanPassiveDiscoveryDeviceId, LanPassiveDiscoveryListenerState, LanPassiveDiscoveryPacket,
    LanPassiveDiscoveryPacketParseError, LanPassiveDiscoveryScanSessionId,
    LanPassiveDiscoverySource, LanPassiveDiscoveryTriggerReason,
    LAN_PASSIVE_DISCOVERY_MAX_PACKET_BYTES, LAN_PASSIVE_DISCOVERY_MAX_SUMMARY_BYTES,
};

pub fn parse_passive_discovery_packet(
    payload: &[u8],
) -> Result<LanPassiveDiscoveryPacket, LanPassiveDiscoveryPacketParseError> {
    if payload.is_empty() {
        return Err(LanPassiveDiscoveryPacketParseError::EmptyPayload);
    }
    if payload.len() > LAN_PASSIVE_DISCOVERY_MAX_PACKET_BYTES {
        return Err(LanPassiveDiscoveryPacketParseError::OversizedPayload {
            payload_len: payload.len(),
            max_payload_len: LAN_PASSIVE_DISCOVERY_MAX_PACKET_BYTES,
        });
    }

    let packet = serde_json::from_slice::<LanPassiveDiscoveryPacketEnvelope>(payload)
        .map_err(|_error| LanPassiveDiscoveryPacketParseError::MalformedPayload)?;
    validate_passive_discovery_packet_envelope(&packet)?;
    Ok(LanPassiveDiscoveryPacket {
        schema_version: packet.schema_version,
        source: packet.source,
        trigger_reason: packet.trigger_reason,
        observed_at: packet.observed_at.trim().to_string(),
        device_id: trim_optional_packet_text(packet.device_id)
            .map(LanPassiveDiscoveryDeviceId::from),
        scan_session_id: trim_optional_packet_text(packet.scan_session_id)
            .map(LanPassiveDiscoveryScanSessionId::from),
        summary: packet.summary.trim().to_string(),
    })
}

fn validate_passive_discovery_packet_envelope(
    packet: &LanPassiveDiscoveryPacketEnvelope,
) -> Result<(), LanPassiveDiscoveryPacketParseError> {
    if packet.schema_version != LanPassiveDiscoveryListenerState::SCHEMA_VERSION {
        return Err(
            LanPassiveDiscoveryPacketParseError::UnsupportedSchemaVersion {
                schema_version: packet.schema_version,
                expected_schema_version: LanPassiveDiscoveryListenerState::SCHEMA_VERSION,
            },
        );
    }
    if packet.observed_at.trim().is_empty() {
        return Err(LanPassiveDiscoveryPacketParseError::EmptyObservedAt);
    }
    let summary = packet.summary.trim();
    if summary.is_empty() {
        return Err(LanPassiveDiscoveryPacketParseError::EmptySummary);
    }
    if summary.len() > LAN_PASSIVE_DISCOVERY_MAX_SUMMARY_BYTES {
        return Err(LanPassiveDiscoveryPacketParseError::OversizedSummary {
            summary_len: summary.len(),
            max_summary_len: LAN_PASSIVE_DISCOVERY_MAX_SUMMARY_BYTES,
        });
    }
    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LanPassiveDiscoveryPacketEnvelope {
    schema_version: u16,
    source: LanPassiveDiscoverySource,
    trigger_reason: LanPassiveDiscoveryTriggerReason,
    observed_at: String,
    #[serde(default)]
    device_id: Option<String>,
    #[serde(default)]
    scan_session_id: Option<String>,
    summary: String,
}

fn trim_optional_packet_text(value: Option<String>) -> Option<String> {
    value
        .map(|text| text.trim().to_string())
        .filter(|text| !text.is_empty())
}
