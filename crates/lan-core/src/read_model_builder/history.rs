use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceReachability, LanPairingProductionDiscoveryState, LanPairingTrustState,
};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanBrowserAddDeviceDiscoveryDevice, LanCanonicalHouseholdDevice, LanDiscoveryEventHistory,
    LanDiscoveryEventHistoryState, LanDiscoveryEventKind, LanDiscoveryEventRow,
    LanDiscoveryEvidenceRecord,
};

use super::history_time::{
    compact_event_identifier, device_discovered_at, discovered_device_label,
    discovered_device_observed_at, earliest_canonical_or_discovered_observed_at,
    evidence_observed_at, latest_canonical_or_discovered_observed_at, reachability_observed_at,
};

pub(super) fn discovery_event_history(
    generated_at: &str,
    unavailable_state: &LanDiscoveryEventHistoryState,
    physical_household_lan_state: &LanPairingProductionDiscoveryState,
    selected_device_readiness: &ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanSelectedDeviceReadiness,
    devices: &[LanCanonicalHouseholdDevice],
    discovered_devices: &[LanBrowserAddDeviceDiscoveryDevice],
) -> LanDiscoveryEventHistory {
    if *unavailable_state == LanDiscoveryEventHistoryState::Unavailable {
        return unavailable_event_history(generated_at);
    }

    let scan_session_id = scan_session_id(generated_at);
    let scan_started_at = earliest_canonical_or_discovered_observed_at(devices, discovered_devices)
        .unwrap_or_else(|| generated_at.to_string());
    let scan_finished_at = latest_canonical_or_discovered_observed_at(devices, discovered_devices)
        .unwrap_or_else(|| generated_at.to_string());
    let mut rows = Vec::new();
    push_scan_started_row(
        &mut rows,
        scan_started_at.as_str(),
        &scan_session_id,
        devices,
        discovered_devices,
    );
    push_discovered_agent_event_rows(&mut rows, &scan_session_id, discovered_devices);
    push_canonical_device_event_rows(&mut rows, generated_at, &scan_session_id, devices);
    push_scan_finished_row(
        &mut rows,
        scan_finished_at.as_str(),
        &scan_session_id,
        devices,
        discovered_devices,
    );
    normalize_discovery_event_rows(&mut rows);

    let latest_event_id = rows.last().map(|row| row.event_id.clone());
    let latest_observed_at = rows.last().map(|row| row.occurred_at.clone());
    let state = history_state(
        &rows,
        physical_household_lan_state,
        selected_device_readiness,
    );
    LanDiscoveryEventHistory {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        generated_at: generated_at.to_string(),
        state,
        latest_event_id,
        latest_observed_at,
        rows,
    }
}

fn unavailable_event_history(generated_at: &str) -> LanDiscoveryEventHistory {
    LanDiscoveryEventHistory {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        generated_at: generated_at.to_string(),
        state: LanDiscoveryEventHistoryState::Unavailable,
        latest_event_id: None,
        latest_observed_at: None,
        rows: Vec::new(),
    }
}

fn push_scan_started_row(
    rows: &mut Vec<LanDiscoveryEventRow>,
    scan_started_at: &str,
    scan_session_id: &str,
    devices: &[LanCanonicalHouseholdDevice],
    discovered_devices: &[LanBrowserAddDeviceDiscoveryDevice],
) {
    if !devices.is_empty() || !discovered_devices.is_empty() {
        push_event_row(
            rows,
            scan_started_at,
            scan_session_id,
            None,
            None,
            LanDiscoveryEventKind::ScanStarted,
            "LAN scan session started".to_string(),
        );
    }
}

fn push_scan_finished_row(
    rows: &mut Vec<LanDiscoveryEventRow>,
    scan_finished_at: &str,
    scan_session_id: &str,
    devices: &[LanCanonicalHouseholdDevice],
    discovered_devices: &[LanBrowserAddDeviceDiscoveryDevice],
) {
    if !devices.is_empty() || !discovered_devices.is_empty() {
        push_event_row(
            rows,
            scan_finished_at,
            scan_session_id,
            None,
            None,
            LanDiscoveryEventKind::ScanFinished,
            "LAN scan session finished".to_string(),
        );
    }
}

fn push_discovered_agent_event_rows(
    rows: &mut Vec<LanDiscoveryEventRow>,
    scan_session_id: &str,
    discovered_devices: &[LanBrowserAddDeviceDiscoveryDevice],
) {
    for device in discovered_devices {
        let Some(agent_status) = device.child_device.agent_status.as_deref() else {
            continue;
        };
        if agent_status.is_empty() {
            continue;
        }
        let observed_at = discovered_device_observed_at(device);
        let label = discovered_device_label(device);
        let summary =
            if crate::network_inventory::api::is_confirmed_agent_status(Some(agent_status)) {
                format!("Detected confirmed agent signature on {label}")
            } else {
                format!("Detected agent signature on {label}")
            };
        push_event_row(
            rows,
            observed_at.as_str(),
            scan_session_id,
            Some(device.child_device.device_id.clone()),
            None,
            LanDiscoveryEventKind::AgentDiscovered,
            summary,
        );
    }
}

fn push_canonical_device_event_rows(
    rows: &mut Vec<LanDiscoveryEventRow>,
    generated_at: &str,
    scan_session_id: &str,
    devices: &[LanCanonicalHouseholdDevice],
) {
    for device in devices {
        push_discovery_device_event(rows, generated_at, scan_session_id, device);
        for evidence in &device.network_identity.evidence_records {
            push_discovery_evidence_event(rows, generated_at, scan_session_id, device, evidence);
        }
    }
}

fn push_discovery_device_event(
    rows: &mut Vec<LanDiscoveryEventRow>,
    fallback_observed_at: &str,
    scan_session_id: &str,
    device: &LanCanonicalHouseholdDevice,
) {
    let event_kind = device_event_kind(device);
    let summary = device_event_summary(&event_kind);
    let evidence_id = device
        .network_identity
        .evidence_records
        .first()
        .map(|record| record.evidence_id.clone());
    let device_occurred_at =
        device_discovered_at(device).unwrap_or_else(|| fallback_observed_at.to_string());
    push_event_row(
        rows,
        device_occurred_at.as_str(),
        scan_session_id,
        Some(device.canonical_device_id.clone()),
        evidence_id.clone(),
        event_kind,
        summary.to_string(),
    );
    push_reachability_event_row(
        rows,
        scan_session_id,
        device,
        evidence_id,
        &device_occurred_at,
    );
}

fn push_reachability_event_row(
    rows: &mut Vec<LanDiscoveryEventRow>,
    scan_session_id: &str,
    device: &LanCanonicalHouseholdDevice,
    evidence_id: Option<String>,
    device_occurred_at: &str,
) {
    if let Some((event_kind, summary)) = reachability_event(&device.network_identity.reachability) {
        let reachability_occurred_at =
            reachability_observed_at(device).unwrap_or_else(|| device_occurred_at.to_string());
        push_event_row(
            rows,
            reachability_occurred_at.as_str(),
            scan_session_id,
            Some(device.canonical_device_id.clone()),
            evidence_id,
            event_kind,
            summary.to_string(),
        );
    }
}

fn device_event_kind(device: &LanCanonicalHouseholdDevice) -> LanDiscoveryEventKind {
    match &device.classification {
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification::ChildAgent => {
            LanDiscoveryEventKind::AgentConfirmed
        }
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification::UnknownLanDevice => {
            LanDiscoveryEventKind::UnknownDetected
        }
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification::Phone
        | ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification::Tablet
        | ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification::Laptop
        | ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification::Desktop
        | ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification::Printer
        | ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification::Television
        | ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification::GameConsole
        | ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification::Camera
        | ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification::NetworkAttachedStorage
        | ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification::InternetOfThings
        | ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification::NetworkInfrastructure
        | ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification::UnsupportedLanDevice => {
            LanDiscoveryEventKind::DeviceFound
        }
    }
}

fn device_event_summary(event_kind: &LanDiscoveryEventKind) -> &'static str {
    match event_kind {
        LanDiscoveryEventKind::AgentConfirmed => "confirmed child agent visible on LAN",
        LanDiscoveryEventKind::UnknownDetected => "unknown LAN device visible for manual review",
        _ => "LAN device visible in discovery snapshot",
    }
}

fn reachability_event(
    reachability: &LanPairingDeviceReachability,
) -> Option<(LanDiscoveryEventKind, &'static str)> {
    match reachability {
        LanPairingDeviceReachability::Online => Some((
            LanDiscoveryEventKind::DeviceOnline,
            "LAN device reachable in current scan",
        )),
        LanPairingDeviceReachability::Offline => Some((
            LanDiscoveryEventKind::DeviceOffline,
            "LAN device offline in current scan",
        )),
        LanPairingDeviceReachability::Stale => Some((
            LanDiscoveryEventKind::DeviceUpdated,
            "LAN device stale in current scan",
        )),
    }
}

fn push_discovery_evidence_event(
    rows: &mut Vec<LanDiscoveryEventRow>,
    fallback_observed_at: &str,
    scan_session_id: &str,
    device: &LanCanonicalHouseholdDevice,
    evidence: &LanDiscoveryEvidenceRecord,
) {
    let summary = format!(
        "evidence {:?} from {:?}",
        evidence.evidence_kind, evidence.source
    );
    push_event_row(
        rows,
        evidence_observed_at(evidence)
            .unwrap_or_else(|| fallback_observed_at.to_string())
            .as_str(),
        scan_session_id,
        Some(device.canonical_device_id.clone()),
        Some(evidence.evidence_id.clone()),
        LanDiscoveryEventKind::EvidenceFound,
        summary,
    );
}

fn push_event_row(
    rows: &mut Vec<LanDiscoveryEventRow>,
    occurred_at: &str,
    scan_session_id: &str,
    affected_device_id: Option<String>,
    evidence_id: Option<String>,
    event_kind: LanDiscoveryEventKind,
    summary: String,
) {
    let previous_event_id = rows.last().map(|row| row.event_id.clone());
    let event_index = rows.len() + 1;
    rows.push(LanDiscoveryEventRow {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        event_id: event_id(scan_session_id, event_index, &event_kind),
        event_kind,
        occurred_at: occurred_at.to_string(),
        previous_event_id,
        scan_session_id: Some(scan_session_id.to_string()),
        affected_device_id,
        evidence_id,
        summary,
    });
}

fn normalize_discovery_event_rows(rows: &mut [LanDiscoveryEventRow]) {
    rows.sort_by(|left, right| left.occurred_at.cmp(&right.occurred_at));
    if let Some(first) = rows.first_mut() {
        first.previous_event_id = None;
    }
    for index in 1..rows.len() {
        let previous_event_id = rows[index - 1].event_id.clone();
        rows[index].previous_event_id = Some(previous_event_id);
    }
}

fn history_state(
    rows: &[LanDiscoveryEventRow],
    physical_household_lan_state: &LanPairingProductionDiscoveryState,
    selected_device_readiness: &ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanSelectedDeviceReadiness,
) -> LanDiscoveryEventHistoryState {
    if has_agent_offline_history_state(selected_device_readiness) {
        LanDiscoveryEventHistoryState::AgentOffline
    } else if rows.is_empty()
        && *physical_household_lan_state == LanPairingProductionDiscoveryState::ManualRequired
    {
        LanDiscoveryEventHistoryState::ManualRequired
    } else if rows.is_empty() {
        LanDiscoveryEventHistoryState::Empty
    } else {
        LanDiscoveryEventHistoryState::Ready
    }
}

fn has_agent_offline_history_state(
    selected_device_readiness: &ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanSelectedDeviceReadiness,
) -> bool {
    selected_device_readiness.selected_child_device_id.is_some()
        && selected_device_readiness.route_id.is_none()
        && selected_device_readiness.trust_state == LanPairingTrustState::Paired
        && selected_device_readiness.reachability == LanPairingDeviceReachability::Online
        && !selected_device_readiness.ready_for_control
}

fn scan_session_id(generated_at: &str) -> String {
    let mut id = String::from("lan-scan-");
    id.push_str(&compact_event_identifier(generated_at));
    id
}

fn event_id(
    scan_session_id: &str,
    event_index: usize,
    event_kind: &LanDiscoveryEventKind,
) -> String {
    format!(
        "{}-{}-{}",
        scan_session_id,
        event_index,
        compact_event_identifier(&format!("{:?}", event_kind))
    )
}
