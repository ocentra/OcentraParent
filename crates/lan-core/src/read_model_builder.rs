pub mod canonical_household_device_spine;
mod production_household_proof;
mod signed_discovery_relay_spine;

use canonical_household_device_spine::canonical_household_devices as compose_canonical_household_devices;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceReachability, LanPairingProductionDiscoveryState, LanPairingTrustState,
    LanTrustedDeviceRegistryEntry,
};
use ocentra_parent_agent_protocol::lan_pairing_authority::LanPairingParentAuthority;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanBrowserAddDeviceDiscoveryDevice, LanBrowserAddDevicePairingRequest,
    LanBrowserAddDeviceReadModel, LanBrowserAddDeviceScanSummary, LanCanonicalHouseholdDevice,
    LanDiscoveryEventHistory, LanDiscoveryEventHistoryState, LanDiscoveryEventKind,
    LanDiscoveryEventRow, LanDiscoveryEvidenceRecord, LanDiscoveryEvidenceSource,
    LanHouseholdDeviceDecision, LanSelectedDeviceReadiness,
};
use std::time::Duration;

use crate::network_inventory::api::{
    is_confirmed_agent_status, is_service_identity_probe_status, service_identity_probe_scan_source,
};
use crate::network_inventory::passive_discovery::current_platform_local_neighbor_collection_summaries;
use crate::read_model::{audit_check_labels, honest_non_claims, lan_discovery_source_matrix};
use production_household_proof::production_household_proof_summary;
use signed_discovery_relay_spine::signed_discovery_relay_spine_summary;

pub struct LanAddDeviceReadModelInput {
    pub generated_at: String,
    pub discovery_source:
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanPairingDiscoverySource,
    pub service_data_available: bool,
    pub platform_data_available: bool,
    pub add_device_state: LanPairingProductionDiscoveryState,
    pub local_service_discovery_state: LanPairingProductionDiscoveryState,
    pub physical_household_lan_state: LanPairingProductionDiscoveryState,
    pub cloud_relay_state: LanPairingProductionDiscoveryState,
    pub discovered_devices: Vec<LanBrowserAddDeviceDiscoveryDevice>,
    pub pairing_requests: Vec<LanBrowserAddDevicePairingRequest>,
    pub trusted_device_registry: Vec<LanTrustedDeviceRegistryEntry>,
    pub household_device_decisions: Vec<LanHouseholdDeviceDecision>,
    pub trusted_device_ids: Vec<String>,
    pub revoked_device_ids: Vec<String>,
    pub selected_device_readiness: LanSelectedDeviceReadiness,
    pub controller_authority: LanPairingParentAuthority,
    pub observer_authority: LanPairingParentAuthority,
}

pub fn build_lan_add_device_read_model(
    input: LanAddDeviceReadModelInput,
) -> LanBrowserAddDeviceReadModel {
    let scan_summary = scan_summary(&input.discovered_devices);
    let canonical_household_devices = compose_canonical_household_devices(
        &input.discovered_devices,
        &input.trusted_device_registry,
        &input.household_device_decisions,
        &input.generated_at,
    );
    let production_household_proof = production_household_proof_summary(
        &input.generated_at,
        input.physical_household_lan_state.clone(),
        &scan_summary,
        &input.trusted_device_registry,
        &input.household_device_decisions,
        &input.selected_device_readiness,
    );
    let signed_discovery_relay_spine = signed_discovery_relay_spine_summary(
        &input.generated_at,
        input.physical_household_lan_state.clone(),
        &scan_summary,
        &input.trusted_device_registry,
        &input.household_device_decisions,
        &input.selected_device_readiness,
    );

    let service_state = if input.service_data_available {
        None
    } else {
        Some(LanPairingProductionDiscoveryState::Unavailable)
    };
    let platform_state = if input.platform_data_available {
        None
    } else {
        Some(LanPairingProductionDiscoveryState::Unavailable)
    };
    let discovery_event_history_state =
        if input.service_data_available || input.platform_data_available {
            LanDiscoveryEventHistoryState::Empty
        } else {
            LanDiscoveryEventHistoryState::Unavailable
        };
    let discovery_event_history = discovery_event_history(
        &input.generated_at,
        &discovery_event_history_state,
        &input.physical_household_lan_state,
        &input.selected_device_readiness,
        &canonical_household_devices,
        &input.discovered_devices,
    );

    LanBrowserAddDeviceReadModel {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        generated_at: input.generated_at.clone(),
        discovery_source: input.discovery_source,
        add_device_state: service_state
            .clone()
            .unwrap_or_else(|| input.add_device_state.clone()),
        local_service_discovery_state: service_state
            .unwrap_or_else(|| input.local_service_discovery_state.clone()),
        physical_household_lan_state: platform_state
            .unwrap_or_else(|| input.physical_household_lan_state.clone()),
        cloud_relay_state: input.cloud_relay_state,
        scan_summary: scan_summary.clone(),
        discovered_devices: input.discovered_devices,
        discovery_event_history,
        canonical_household_devices,
        pairing_requests: input.pairing_requests,
        trusted_device_registry: input.trusted_device_registry,
        household_device_decisions: input.household_device_decisions,
        production_household_proof: Some(production_household_proof),
        signed_discovery_relay_spine: Some(signed_discovery_relay_spine),
        lan_discovery_source_matrix: Some(lan_discovery_source_matrix(
            &input.generated_at,
            &scan_summary,
        )),
        trusted_device_ids: input.trusted_device_ids,
        revoked_device_ids: input.revoked_device_ids,
        selected_device_readiness: input.selected_device_readiness,
        controller_authority: input.controller_authority,
        observer_authority: input.observer_authority,
        route_requirement_labels: constants::lan_pairing::ROUTE_REQUIREMENTS
            .iter()
            .map(|requirement| (*requirement).to_string())
            .collect(),
        audit_check_labels: audit_check_labels(),
        honest_non_claims: honest_non_claims(),
    }
}

pub fn canonical_household_devices(
    discovered_devices: &[LanBrowserAddDeviceDiscoveryDevice],
    trusted_registry: &[LanTrustedDeviceRegistryEntry],
    household_device_decisions: &[LanHouseholdDeviceDecision],
    observed_at: &str,
) -> Vec<LanCanonicalHouseholdDevice> {
    compose_canonical_household_devices(
        discovered_devices,
        trusted_registry,
        household_device_decisions,
        observed_at,
    )
}

fn scan_summary(devices: &[LanBrowserAddDeviceDiscoveryDevice]) -> LanBrowserAddDeviceScanSummary {
    let agent_device_count = devices.iter().filter(|device| has_agent(device)).count() as u32;
    let infrastructure_device_count = devices
        .iter()
        .filter(|device| is_infrastructure(device))
        .count() as u32;
    let passive_device_count = devices
        .iter()
        .filter(|device| {
            device.discovery_status
                == ocentra_parent_agent_protocol::lan_pairing::LanPairingDiscoveryRuntimeStatus::NetworkNeighbor
                && !is_infrastructure(device)
        })
        .count() as u32;
    let unsupported_device_count =
        devices.iter().filter(|device| !has_agent(device)).count() as u32;

    LanBrowserAddDeviceScanSummary {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        source_labels: scan_source_labels(devices),
        scanned_device_count: devices.len() as u32,
        agent_device_count,
        passive_device_count,
        infrastructure_device_count,
        unsupported_device_count,
        passive_local_neighbor_collection_summaries:
            current_platform_local_neighbor_collection_summaries(Duration::from_millis(250)),
    }
}

fn discovery_event_history(
    generated_at: &str,
    unavailable_state: &LanDiscoveryEventHistoryState,
    physical_household_lan_state: &LanPairingProductionDiscoveryState,
    selected_device_readiness: &LanSelectedDeviceReadiness,
    devices: &[LanCanonicalHouseholdDevice],
    discovered_devices: &[LanBrowserAddDeviceDiscoveryDevice],
) -> LanDiscoveryEventHistory {
    if *unavailable_state == LanDiscoveryEventHistoryState::Unavailable {
        return LanDiscoveryEventHistory {
            schema_version: constants::lan_pairing::SCHEMA_VERSION,
            generated_at: generated_at.to_string(),
            state: LanDiscoveryEventHistoryState::Unavailable,
            latest_event_id: None,
            latest_observed_at: None,
            rows: Vec::new(),
        };
    }

    let scan_session_id = scan_session_id(generated_at);
    let scan_started_at = earliest_canonical_or_discovered_observed_at(devices, discovered_devices)
        .unwrap_or_else(|| generated_at.to_string());
    let scan_finished_at = latest_canonical_or_discovered_observed_at(devices, discovered_devices)
        .unwrap_or_else(|| generated_at.to_string());
    let mut rows = Vec::new();
    if !devices.is_empty() || !discovered_devices.is_empty() {
        push_event_row(
            &mut rows,
            scan_started_at.as_str(),
            &scan_session_id,
            None,
            None,
            LanDiscoveryEventKind::ScanStarted,
            "LAN scan session started".to_string(),
        );
    }
    push_discovered_agent_event_rows(&mut rows, &scan_session_id, discovered_devices);
    for device in devices {
        push_discovery_device_event(&mut rows, generated_at, &scan_session_id, device);
        for evidence in &device.network_identity.evidence_records {
            push_discovery_evidence_event(
                &mut rows,
                generated_at,
                &scan_session_id,
                device,
                evidence,
            );
        }
    }
    if !devices.is_empty() || !discovered_devices.is_empty() {
        push_event_row(
            &mut rows,
            scan_finished_at.as_str(),
            &scan_session_id,
            None,
            None,
            LanDiscoveryEventKind::ScanFinished,
            "LAN scan session finished".to_string(),
        );
    }
    normalize_discovery_event_rows(&mut rows);

    let latest_event_id = rows.last().map(|row| row.event_id.clone());
    let latest_observed_at = rows.last().map(|row| row.occurred_at.clone());
    let state = if has_agent_offline_history_state(selected_device_readiness) {
        LanDiscoveryEventHistoryState::AgentOffline
    } else if rows.is_empty()
        && *physical_household_lan_state == LanPairingProductionDiscoveryState::ManualRequired
    {
        LanDiscoveryEventHistoryState::ManualRequired
    } else if rows.is_empty() {
        LanDiscoveryEventHistoryState::Empty
    } else {
        LanDiscoveryEventHistoryState::Ready
    };

    LanDiscoveryEventHistory {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        generated_at: generated_at.to_string(),
        state,
        latest_event_id,
        latest_observed_at,
        rows,
    }
}

fn has_agent_offline_history_state(selected_device_readiness: &LanSelectedDeviceReadiness) -> bool {
    selected_device_readiness.selected_child_device_id.is_some()
        && selected_device_readiness.route_id.is_none()
        && selected_device_readiness.trust_state == LanPairingTrustState::Paired
        && selected_device_readiness.reachability == LanPairingDeviceReachability::Online
        && !selected_device_readiness.ready_for_control
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
        let summary = if is_confirmed_agent_status(Some(agent_status)) {
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

fn push_discovery_device_event(
    rows: &mut Vec<LanDiscoveryEventRow>,
    fallback_observed_at: &str,
    scan_session_id: &str,
    device: &LanCanonicalHouseholdDevice,
) {
    let event_kind = match &device.classification {
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
        |
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification::NetworkInfrastructure
        | ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification::UnsupportedLanDevice => {
            LanDiscoveryEventKind::DeviceFound
        }
    };
    let summary = match &event_kind {
        LanDiscoveryEventKind::AgentConfirmed => "confirmed child agent visible on LAN",
        LanDiscoveryEventKind::UnknownDetected => "unknown LAN device visible for manual review",
        _ => "LAN device visible in discovery snapshot",
    };
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
    if let Some((event_kind, summary)) = reachability_event(&device.network_identity.reachability) {
        let reachability_occurred_at =
            reachability_observed_at(device).unwrap_or_else(|| device_occurred_at.clone());
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

fn compact_event_identifier(value: &str) -> String {
    value
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

fn earliest_device_observed_at(devices: &[LanCanonicalHouseholdDevice]) -> Option<String> {
    let mut earliest: Option<String> = None;
    for device in devices {
        if let Some(observed_at) = device_discovered_at(device) {
            if earliest
                .as_ref()
                .is_none_or(|current| observed_at.as_str() < current.as_str())
            {
                earliest = Some(observed_at);
            }
        }
        for evidence in &device.network_identity.evidence_records {
            if let Some(observed_at) = evidence_observed_at(evidence) {
                if earliest
                    .as_ref()
                    .is_none_or(|current| observed_at.as_str() < current.as_str())
                {
                    earliest = Some(observed_at);
                }
            }
        }
    }
    earliest
}

fn latest_device_observed_at(devices: &[LanCanonicalHouseholdDevice]) -> Option<String> {
    let mut latest: Option<String> = None;
    for device in devices {
        if let Some(observed_at) =
            reachability_observed_at(device).or_else(|| device_discovered_at(device))
        {
            if latest
                .as_ref()
                .is_none_or(|current| observed_at.as_str() > current.as_str())
            {
                latest = Some(observed_at);
            }
        }
        for evidence in &device.network_identity.evidence_records {
            if let Some(observed_at) = latest_evidence_observed_at(evidence) {
                if latest
                    .as_ref()
                    .is_none_or(|current| observed_at.as_str() > current.as_str())
                {
                    latest = Some(observed_at);
                }
            }
        }
    }
    latest
}

fn earliest_canonical_or_discovered_observed_at(
    devices: &[LanCanonicalHouseholdDevice],
    discovered_devices: &[LanBrowserAddDeviceDiscoveryDevice],
) -> Option<String> {
    let canonical = earliest_device_observed_at(devices);
    let discovered = earliest_discovered_device_observed_at(discovered_devices);
    earliest_timestamp(canonical.as_deref(), discovered.as_deref())
}

fn latest_canonical_or_discovered_observed_at(
    devices: &[LanCanonicalHouseholdDevice],
    discovered_devices: &[LanBrowserAddDeviceDiscoveryDevice],
) -> Option<String> {
    let canonical = latest_device_observed_at(devices);
    let discovered = latest_discovered_device_observed_at(discovered_devices);
    latest_timestamp(canonical.as_deref(), discovered.as_deref())
}

fn device_discovered_at(device: &LanCanonicalHouseholdDevice) -> Option<String> {
    let mut earliest: Option<String> = None;
    for evidence in &device.network_identity.evidence_records {
        if let Some(observed_at) = evidence_observed_at(evidence) {
            if earliest
                .as_ref()
                .is_none_or(|current| observed_at.as_str() < current.as_str())
            {
                earliest = Some(observed_at);
            }
        }
    }
    earliest
}

fn discovered_device_observed_at(device: &LanBrowserAddDeviceDiscoveryDevice) -> String {
    if !device.discovered_at.is_empty() {
        device.discovered_at.clone()
    } else {
        device_label_timestamp_fallback(device)
    }
}

fn discovered_device_label(device: &LanBrowserAddDeviceDiscoveryDevice) -> String {
    device
        .child_device
        .hostname
        .as_ref()
        .filter(|hostname| !hostname.is_empty())
        .cloned()
        .unwrap_or_else(|| device.child_device.label.clone())
}

fn earliest_discovered_device_observed_at(
    discovered_devices: &[LanBrowserAddDeviceDiscoveryDevice],
) -> Option<String> {
    let mut earliest: Option<String> = None;
    for device in discovered_devices {
        let observed_at = discovered_device_observed_at(device);
        if earliest
            .as_ref()
            .is_none_or(|current| observed_at.as_str() < current.as_str())
        {
            earliest = Some(observed_at);
        }
    }
    earliest
}

fn latest_discovered_device_observed_at(
    discovered_devices: &[LanBrowserAddDeviceDiscoveryDevice],
) -> Option<String> {
    let mut latest: Option<String> = None;
    for device in discovered_devices {
        let observed_at = discovered_device_observed_at(device);
        if latest
            .as_ref()
            .is_none_or(|current| observed_at.as_str() > current.as_str())
        {
            latest = Some(observed_at);
        }
    }
    latest
}

fn earliest_timestamp(first: Option<&str>, second: Option<&str>) -> Option<String> {
    match (first, second) {
        (Some(first), Some(second)) => {
            Some(if first <= second { first } else { second }.to_string())
        }
        (Some(first), None) => Some(first.to_string()),
        (None, Some(second)) => Some(second.to_string()),
        (None, None) => None,
    }
}

fn latest_timestamp(first: Option<&str>, second: Option<&str>) -> Option<String> {
    match (first, second) {
        (Some(first), Some(second)) => {
            Some(if first >= second { first } else { second }.to_string())
        }
        (Some(first), None) => Some(first.to_string()),
        (None, Some(second)) => Some(second.to_string()),
        (None, None) => None,
    }
}

fn device_label_timestamp_fallback(device: &LanBrowserAddDeviceDiscoveryDevice) -> String {
    let mut fallback = String::from("undated-");
    fallback.push_str(&compact_event_identifier(&device.child_device.device_id));
    fallback
}

fn reachability_observed_at(device: &LanCanonicalHouseholdDevice) -> Option<String> {
    match device.network_identity.reachability {
        LanPairingDeviceReachability::Online => {
            latest_evidence_last_seen(&device.network_identity.evidence_records)
                .or_else(|| device_discovered_at(device))
        }
        LanPairingDeviceReachability::Offline => device
            .network_identity
            .offline_at
            .clone()
            .or_else(|| latest_evidence_last_seen(&device.network_identity.evidence_records))
            .or_else(|| device_discovered_at(device)),
        LanPairingDeviceReachability::Stale => device
            .network_identity
            .stale_at
            .clone()
            .or_else(|| latest_evidence_last_seen(&device.network_identity.evidence_records))
            .or_else(|| device_discovered_at(device)),
    }
}

fn evidence_observed_at(evidence: &LanDiscoveryEvidenceRecord) -> Option<String> {
    if !evidence.first_seen_at.is_empty() {
        Some(evidence.first_seen_at.clone())
    } else if !evidence.last_seen_at.is_empty() {
        Some(evidence.last_seen_at.clone())
    } else {
        None
    }
}

fn latest_evidence_last_seen(evidence_records: &[LanDiscoveryEvidenceRecord]) -> Option<String> {
    let mut latest: Option<String> = None;
    for evidence in evidence_records {
        let Some(observed_at) = latest_evidence_observed_at(evidence) else {
            continue;
        };
        if latest
            .as_ref()
            .is_none_or(|current| observed_at.as_str() > current.as_str())
        {
            latest = Some(observed_at);
        }
    }
    latest
}

fn latest_evidence_observed_at(evidence: &LanDiscoveryEvidenceRecord) -> Option<String> {
    if !evidence.last_seen_at.is_empty() {
        Some(evidence.last_seen_at.clone())
    } else {
        evidence_observed_at(evidence)
    }
}

fn scan_source_labels(devices: &[LanBrowserAddDeviceDiscoveryDevice]) -> Vec<String> {
    let mut labels = vec![constants::lan_pairing::LAN_SCAN_SOURCE_LOCAL_SERVICE.to_string()];
    for label in devices
        .iter()
        .flat_map(|device| device.evidence_sources.iter())
        .filter_map(scan_source_label)
    {
        if !labels.iter().any(|existing| existing == label) {
            labels.push(label.to_string());
        }
    }
    if devices
        .iter()
        .any(|device| is_service_identity_probe_status(device.child_device.agent_status.as_deref()))
        && !labels
            .iter()
            .any(|label| label == service_identity_probe_scan_source())
    {
        labels.push(service_identity_probe_scan_source().to_string());
    }
    if devices.iter().any(|device| {
        device.service_identity_probe_evidence.iter().any(|evidence| {
            matches!(
                evidence.evidence_kind,
                ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanServiceIdentityProbeEvidenceKind::SnmpSysDescr
                    | ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanServiceIdentityProbeEvidenceKind::SnmpSysName
            )
        })
    }) && !labels
        .iter()
        .any(|label| label == constants::lan_pairing::LAN_SCAN_SOURCE_ALLOWED_SNMP_RESPONSE)
    {
        labels.push(constants::lan_pairing::LAN_SCAN_SOURCE_ALLOWED_SNMP_RESPONSE.to_string());
    }
    if devices.iter().any(|device| {
        device
            .hint_sources
            .contains(&LanDiscoveryEvidenceSource::PreviousScanSnapshot)
    }) {
        labels.push(constants::lan_pairing::LAN_SCAN_SOURCE_PREVIOUS_SCAN_SNAPSHOT.to_string());
    }
    labels
}

fn scan_source_label(source: &LanDiscoveryEvidenceSource) -> Option<&'static str> {
    match source {
        LanDiscoveryEvidenceSource::LocalService => {
            Some(constants::lan_pairing::LAN_SCAN_SOURCE_LOCAL_SERVICE)
        }
        LanDiscoveryEvidenceSource::WindowsNeighborTable => {
            Some(constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR)
        }
        LanDiscoveryEvidenceSource::LinuxProcNetArp => {
            Some(constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_PROC_NET_ARP)
        }
        LanDiscoveryEvidenceSource::LinuxIpNeigh => {
            Some(constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_IP_NEIGH)
        }
        LanDiscoveryEvidenceSource::MacosArp => {
            Some(constants::lan_pairing::LAN_SCAN_SOURCE_MACOS_ARP)
        }
        LanDiscoveryEvidenceSource::ServiceIdentityProbe => {
            Some(constants::lan_pairing::LAN_SCAN_SOURCE_SERVICE_IDENTITY_PROBE)
        }
        LanDiscoveryEvidenceSource::MdnsDnsSdQuery => {
            Some(constants::lan_pairing::LAN_SCAN_SOURCE_MDNS_DNS_SD)
        }
        LanDiscoveryEvidenceSource::SsdpUpnpQuery => {
            Some(constants::lan_pairing::LAN_SCAN_SOURCE_SSDP_UPNP)
        }
        LanDiscoveryEvidenceSource::DnsCache => {
            Some(constants::lan_pairing::LAN_SCAN_SOURCE_DNS_CACHE)
        }
        LanDiscoveryEvidenceSource::Netbios => {
            Some(constants::lan_pairing::LAN_SCAN_SOURCE_NETBIOS)
        }
        LanDiscoveryEvidenceSource::Llmnr => Some(constants::lan_pairing::LAN_SCAN_SOURCE_LLMNR),
        LanDiscoveryEvidenceSource::PreviousScanSnapshot
        | LanDiscoveryEvidenceSource::TrustedRegistry
        | LanDiscoveryEvidenceSource::ParentAssignment
        | LanDiscoveryEvidenceSource::ChildAgentHello
        | LanDiscoveryEvidenceSource::ChildAgentHeartbeat => None,
    }
}

fn has_agent(device: &LanBrowserAddDeviceDiscoveryDevice) -> bool {
    is_confirmed_agent_status(device.child_device.agent_status.as_deref())
}

fn is_infrastructure(device: &LanBrowserAddDeviceDiscoveryDevice) -> bool {
    device.child_device.platform == constants::lan_pairing::PLATFORM_ROUTER
}
