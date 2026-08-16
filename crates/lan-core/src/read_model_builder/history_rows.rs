use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanBrowserAddDeviceDiscoveryDevice, LanCanonicalHouseholdDevice, LanDiscoveryEventKind,
    LanDiscoveryEventRow, LanDiscoveryEvidenceRecord,
};

pub(super) fn push_scan_started_row(
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

pub(super) fn push_scan_finished_row(
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

pub(super) fn push_discovered_agent_event_rows(
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
        let observed_at = super::super::history_time::discovered_device_observed_at(device);
        let label = super::super::history_time::discovered_device_label(device);
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

pub(super) fn push_canonical_device_event_rows(
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
    let event_kind = super::event_kind::device_event_kind(device);
    let summary = super::event_kind::device_event_summary(&event_kind);
    let evidence_id = device
        .network_identity
        .evidence_records
        .first()
        .map(|record| record.evidence_id.clone());
    let device_occurred_at = super::super::history_time::device_discovered_at(device)
        .unwrap_or_else(|| fallback_observed_at.to_string());
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
    if let Some((event_kind, summary)) =
        super::event_kind::reachability_event(&device.network_identity.reachability)
    {
        let reachability_occurred_at = super::super::history_time::reachability_observed_at(device)
            .unwrap_or_else(|| device_occurred_at.to_string());
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
        super::super::history_time::evidence_observed_at(evidence)
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

fn event_id(
    scan_session_id: &str,
    event_index: usize,
    event_kind: &LanDiscoveryEventKind,
) -> String {
    format!(
        "{}-{}-{}",
        scan_session_id,
        event_index,
        super::super::history_time::compact_event_identifier(&format!("{:?}", event_kind))
    )
}

pub(super) fn normalize_discovery_event_rows(rows: &mut [LanDiscoveryEventRow]) {
    rows.sort_by(|left, right| left.occurred_at.cmp(&right.occurred_at));
    if let Some(first) = rows.first_mut() {
        first.previous_event_id = None;
    }
    for index in 1..rows.len() {
        let previous_event_id = rows[index - 1].event_id.clone();
        rows[index].previous_event_id = Some(previous_event_id);
    }
}
