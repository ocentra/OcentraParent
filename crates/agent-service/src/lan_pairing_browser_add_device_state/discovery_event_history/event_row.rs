use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingText;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanDiscoveryEventKind, LanDiscoveryEventRow,
};

const LAN_DISCOVERY_EVENT_PREFIX: &str = "lan-discovery-";
const LAN_DISCOVERY_OBSERVED_SUMMARY_PREFIX: &str = "Observed ";
const LAN_DISCOVERY_EVIDENCE_FOR_SEPARATOR: &str = " evidence for ";
const LAN_DISCOVERY_AGENT_CONFIRMED_SUMMARY_PREFIX: &str = "Confirmed child agent inventory for ";

pub(super) fn discovery_event_row(
    event_id: LanPairingText,
    event_kind: LanDiscoveryEventKind,
    occurred_at: LanPairingText,
    scan_session_id: Option<LanPairingText>,
    affected_device_id: Option<LanPairingText>,
    evidence_id: Option<LanPairingText>,
    summary: LanPairingText,
) -> LanDiscoveryEventRow {
    LanDiscoveryEventRow {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        event_id: event_id.0,
        event_kind,
        occurred_at: occurred_at.0,
        previous_event_id: None,
        scan_session_id: scan_session_id.map(|value| value.0),
        affected_device_id: affected_device_id.map(|value| value.0),
        evidence_id: evidence_id.map(|value| value.0),
        summary: summary.0,
    }
}

pub(super) fn push_discovery_event_row(
    rows: &mut Vec<LanDiscoveryEventRow>,
    row: LanDiscoveryEventRow,
) {
    if rows
        .iter()
        .any(|existing| existing.event_id == row.event_id)
    {
        return;
    }
    rows.push(row);
}

pub(super) fn keyed_discovery_event_id(
    prefix: LanPairingText,
    scan_key: &LanPairingText,
    entity_id: &LanPairingText,
) -> LanPairingText {
    let mut event_id = prefix.0;
    event_id.push_str(&scan_key.0);
    event_id.push('-');
    event_id.push_str(&entity_id.0);
    LanPairingText(event_id)
}

pub(super) fn reachability_discovery_event_id(
    segment: &LanPairingText,
    scan_key: &LanPairingText,
    entity_id: &LanPairingText,
) -> LanPairingText {
    let mut event_id = String::from(LAN_DISCOVERY_EVENT_PREFIX);
    event_id.push_str(segment.0.as_str());
    event_id.push('-');
    event_id.push_str(&scan_key.0);
    event_id.push('-');
    event_id.push_str(&entity_id.0);
    LanPairingText(event_id)
}

pub(super) fn evidence_found_summary(
    evidence_kind: &LanPairingText,
    display_name: &LanPairingText,
) -> LanPairingText {
    let mut summary = String::from(LAN_DISCOVERY_OBSERVED_SUMMARY_PREFIX);
    summary.push_str(&evidence_kind.0);
    summary.push_str(LAN_DISCOVERY_EVIDENCE_FOR_SEPARATOR);
    summary.push_str(&display_name.0);
    LanPairingText(summary)
}

pub(super) fn agent_confirmed_summary(display_name: &LanPairingText) -> LanPairingText {
    let mut summary = String::from(LAN_DISCOVERY_AGENT_CONFIRMED_SUMMARY_PREFIX);
    summary.push_str(&display_name.0);
    LanPairingText(summary)
}
